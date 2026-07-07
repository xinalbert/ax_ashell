#!/usr/bin/env python3
from __future__ import annotations

import argparse
import datetime as dt
import re
from dataclasses import dataclass
from pathlib import Path


CANONICAL_TAG_PATTERN = re.compile(r"^v(?P<cargo_version>\d+\.\d+\.\d+(?:-\d+)?)$")
LEGACY_TAG_PATTERN = re.compile(
    r"^v(?P<year>\d{4})\.(?P<month>\d{2})\.(?P<day>\d{2})(?:\.(?P<suffix>\d+))?$"
)


@dataclass(frozen=True)
class ReleaseVersion:
    cargo_version: str
    public_version: str
    bundle_short_version: str
    bundle_version: str
    tag: str
    suffix: str


def build_bundle_version(year: int, month: int, day: int, suffix: str) -> str:
    bundle_version = f"{year:04d}{month:02d}{day:02d}"
    if suffix:
        bundle_version = f"{bundle_version}.{suffix}"
    return bundle_version


def validate_suffix(raw_tag_or_version: str, suffix: str) -> None:
    if not suffix:
        return
    if not suffix.isdigit():
        raise ValueError(
            f"unsupported version format: {raw_tag_or_version!r}; numeric suffix is required"
        )
    if suffix.startswith("0") and suffix != "0":
        raise ValueError(
            f"unsupported version format: {raw_tag_or_version!r}; numeric suffix must not contain leading zeros"
        )


def validate_date(raw_tag_or_version: str, year: int, month: int, day: int) -> None:
    try:
        dt.date(year, month, day)
    except ValueError as exc:
        raise ValueError(f"unsupported version format: {raw_tag_or_version!r}; {exc}") from exc


def release_version_from_tag(tag: str) -> ReleaseVersion:
    canonical_match = CANONICAL_TAG_PATTERN.fullmatch(tag)
    if canonical_match:
        version = release_version_from_cargo(canonical_match.group("cargo_version"))
        return ReleaseVersion(
            cargo_version=version.cargo_version,
            public_version=version.public_version,
            bundle_short_version=version.bundle_short_version,
            bundle_version=version.bundle_version,
            tag=tag,
            suffix=version.suffix,
        )

    legacy_match = LEGACY_TAG_PATTERN.fullmatch(tag)
    if not legacy_match:
        raise ValueError(
            f"unsupported tag format: {tag!r}; expected vYYYY.M.D or vYYYY.M.D-N"
        )

    year = int(legacy_match.group("year"))
    month = int(legacy_match.group("month"))
    day = int(legacy_match.group("day"))
    suffix = legacy_match.group("suffix") or ""
    validate_suffix(tag, suffix)
    validate_date(tag, year, month, day)

    base_cargo = f"{year}.{month}.{day}"
    public_version = f"{year:04d}.{month:02d}.{day:02d}"
    bundle_version = build_bundle_version(year, month, day, suffix)
    cargo_version = base_cargo
    if suffix:
        public_version = f"{public_version}.{suffix}"
        cargo_version = f"{cargo_version}-{suffix}"

    return ReleaseVersion(
        cargo_version=cargo_version,
        public_version=public_version,
        bundle_short_version=f"{year:04d}.{month:02d}.{day:02d}",
        bundle_version=bundle_version,
        tag=tag,
        suffix=suffix,
    )


def release_version_from_cargo(cargo_version: str) -> ReleaseVersion:
    version = cargo_version.strip()
    if not version:
        raise ValueError("cargo version is empty")

    core, dash, suffix = version.partition("-")
    parts = core.split(".")
    if len(parts) != 3 or not all(part.isdigit() for part in parts):
        raise ValueError(
            f"unsupported Cargo version format: {cargo_version!r}; expected YYYY.M.D or YYYY.M.D-N"
        )

    year, month, day = (int(part) for part in parts)
    if dash and not suffix:
        raise ValueError(
            f"unsupported Cargo version format: {cargo_version!r}; expected YYYY.M.D or YYYY.M.D-N"
        )
    validate_suffix(cargo_version, suffix if dash else "")
    validate_date(cargo_version, year, month, day)

    public_version = f"{year:04d}.{month:02d}.{day:02d}"
    if dash and suffix:
        public_version = f"{public_version}.{suffix}"

    return ReleaseVersion(
        cargo_version=version,
        public_version=public_version,
        bundle_short_version=f"{year:04d}.{month:02d}.{day:02d}",
        bundle_version=build_bundle_version(year, month, day, suffix if dash else ""),
        tag=f"v{version}",
        suffix=suffix if dash else "",
    )


def read_cargo_manifest_version(path: Path) -> str:
    in_package = False
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped.startswith("["):
            in_package = stripped == "[package]"
            continue
        if in_package and stripped.startswith("version = "):
            parts = stripped.split('"')
            if len(parts) >= 2:
                return parts[1]
            break
    raise ValueError(f"could not read package version from {path}")


def update_cargo_manifest_version(path: Path, next_version: str) -> bool:
    lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
    in_package = False
    changed = False

    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("["):
            in_package = stripped == "[package]"
            continue
        if in_package and stripped.startswith("version = "):
            indent = line[: len(line) - len(line.lstrip())]
            newline = "\n" if line.endswith("\n") else ""
            lines[index] = f'{indent}version = "{next_version}"{newline}'
            changed = True
            break

    if not changed:
        raise ValueError(f"could not update package version in {path}")

    next_text = "".join(lines)
    if next_text != path.read_text(encoding="utf-8"):
        path.write_text(next_text, encoding="utf-8")
        return True
    return False


def update_cargo_lock_version(path: Path, package_name: str, next_version: str) -> bool:
    lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
    in_package = False
    current_name: str | None = None
    changed = False

    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped == "[[package]]":
            in_package = True
            current_name = None
            continue
        if in_package and stripped.startswith("name = "):
            parts = stripped.split('"')
            current_name = parts[1] if len(parts) >= 2 else None
            continue
        if in_package and stripped.startswith("version = ") and current_name == package_name:
            indent = line[: len(line) - len(line.lstrip())]
            newline = "\n" if line.endswith("\n") else ""
            lines[index] = f'{indent}version = "{next_version}"{newline}'
            changed = True
            break

    if not changed:
        raise ValueError(f"could not update root package version in {path}")

    next_text = "".join(lines)
    if next_text != path.read_text(encoding="utf-8"):
        path.write_text(next_text, encoding="utf-8")
        return True
    return False


def resolve_version(args: argparse.Namespace) -> ReleaseVersion:
    if args.tag:
        return release_version_from_tag(args.tag)
    if args.cargo_version:
        return release_version_from_cargo(args.cargo_version)
    if args.cargo_version_file:
        return release_version_from_cargo(read_cargo_manifest_version(Path(args.cargo_version_file)))
    raise ValueError("one of --tag, --cargo-version or --cargo-version-file is required")


def cmd_env(args: argparse.Namespace) -> int:
    version = resolve_version(args)
    print(f"RELEASE_CARGO_VERSION={version.cargo_version}")
    print(f"RELEASE_PUBLIC_VERSION={version.public_version}")
    print(f"RELEASE_BUNDLE_SHORT_VERSION={version.bundle_short_version}")
    print(f"RELEASE_BUNDLE_VERSION={version.bundle_version}")
    print(f"RELEASE_TAG={version.tag}")
    print(f"RELEASE_SUFFIX={version.suffix}")
    return 0


def cmd_sync_manifest(args: argparse.Namespace) -> int:
    version = resolve_version(args)
    cargo_toml = Path(args.cargo_toml)
    cargo_lock = Path(args.cargo_lock)

    update_cargo_manifest_version(cargo_toml, version.cargo_version)
    update_cargo_lock_version(cargo_lock, args.package_name, version.cargo_version)
    print(version.cargo_version)
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Resolve and synchronize AxShell release versions."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    env_parser = subparsers.add_parser(
        "env", help="print derived release version fields as KEY=VALUE lines"
    )
    add_resolve_arguments(env_parser)
    env_parser.set_defaults(func=cmd_env)

    sync_parser = subparsers.add_parser(
        "sync-manifest",
        help="rewrite Cargo.toml and Cargo.lock root package version from a resolved release version",
    )
    add_resolve_arguments(sync_parser)
    sync_parser.add_argument("--cargo-toml", default="Cargo.toml")
    sync_parser.add_argument("--cargo-lock", default="Cargo.lock")
    sync_parser.add_argument("--package-name", default="ax_shell")
    sync_parser.set_defaults(func=cmd_sync_manifest)

    return parser


def add_resolve_arguments(parser: argparse.ArgumentParser) -> None:
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--tag")
    group.add_argument("--cargo-version")
    group.add_argument("--cargo-version-file")


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    try:
        return args.func(args)
    except ValueError as exc:
        parser.error(str(exc))


if __name__ == "__main__":
    raise SystemExit(main())
