use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fs, io,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

#[cfg(target_os = "macos")]
use std::time::Duration;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use gpui::{Context, Image, ImageFormat};
use serde::{Deserialize, Serialize};

const CACHE_VERSION: u32 = 1;
#[cfg(target_os = "macos")]
const MACOS_ICONS_PER_TICK: usize = 4;
#[cfg(any(target_os = "windows", target_os = "linux"))]
const ICON_SIZE: u16 = 32;

const COMMON_EXTENSIONS: &[&str] = &[
    "txt", "md", "rtf", "log", "csv", "tsv", "json", "yaml", "yml", "toml", "xml", "ini", "cfg",
    "conf", "rs", "py", "rb", "go", "java", "kt", "c", "h", "cpp", "hpp", "cs", "php", "swift",
    "sh", "zsh", "fish", "ps1", "js", "mjs", "cjs", "ts", "tsx", "jsx", "vue", "html", "htm",
    "css", "scss", "sass", "less", "sql", "pdf", "doc", "docx", "odt", "xls", "xlsx", "ods", "ppt",
    "pptx", "odp", "zip", "tar", "gz", "bz2", "xz", "7z", "rar", "iso", "png", "jpg", "jpeg",
    "gif", "webp", "svg", "bmp", "ico", "tif", "tiff", "heic", "mp3", "wav", "flac", "ogg", "m4a",
    "mp4", "mkv", "mov", "avi", "webm", "exe", "msi", "app", "dmg", "pkg", "deb", "rpm",
];

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FileIconKey {
    Directory,
    GenericFile,
    FileExtension(String),
}

impl FileIconKey {
    fn cache_key(&self) -> String {
        match self {
            Self::Directory => "directory".to_string(),
            Self::GenericFile => "file".to_string(),
            Self::FileExtension(extension) => format!("extension:{extension}"),
        }
    }

    fn from_cache_key(value: &str) -> Option<Self> {
        if value == "directory" {
            return Some(Self::Directory);
        }
        if value == "file" {
            return Some(Self::GenericFile);
        }
        let extension = value.strip_prefix("extension:")?;
        (!extension.is_empty()).then(|| Self::FileExtension(extension.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CacheIdentity {
    platform: String,
    theme: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredFileIconCache {
    version: u32,
    platform: String,
    #[serde(default)]
    theme: Option<String>,
    keys: Vec<String>,
    icons: BTreeMap<String, StoredIcon>,
}

impl StoredFileIconCache {
    fn empty(identity: &CacheIdentity) -> Self {
        Self {
            version: CACHE_VERSION,
            platform: identity.platform.clone(),
            theme: identity.theme.clone(),
            keys: cache_keys()
                .into_iter()
                .map(|key| key.cache_key())
                .collect(),
            icons: BTreeMap::new(),
        }
    }

    fn is_compatible(&self, identity: &CacheIdentity) -> bool {
        self.version == CACHE_VERSION
            && self.platform == identity.platform
            && self.theme == identity.theme
            && self.keys
                == cache_keys()
                    .into_iter()
                    .map(|key| key.cache_key())
                    .collect::<Vec<_>>()
            && self.icons.iter().all(|(key, icon)| {
                FileIconKey::from_cache_key(key).is_some_and(|key| cache_keys().contains(&key))
                    && icon.decode().is_some()
            })
    }

    fn resolved_images(&self) -> HashMap<FileIconKey, Arc<Image>> {
        self.icons
            .iter()
            .filter_map(|(key, icon)| {
                let key = FileIconKey::from_cache_key(key)?;
                let image = icon.decode()?;
                Some((key, Arc::new(image)))
            })
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredIcon {
    format: StoredIconFormat,
    bytes: String,
}

impl StoredIcon {
    fn from_icon_data(icon: IconData) -> Self {
        Self {
            format: icon.format,
            bytes: BASE64.encode(icon.bytes),
        }
    }

    fn decode(&self) -> Option<Image> {
        Some(Image::from_bytes(
            self.format.image_format(),
            BASE64.decode(&self.bytes).ok()?,
        ))
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum StoredIconFormat {
    Png,
    Jpeg,
    Webp,
    Gif,
    Svg,
    Bmp,
    Ico,
}

impl StoredIconFormat {
    fn image_format(self) -> ImageFormat {
        match self {
            Self::Png => ImageFormat::Png,
            Self::Jpeg => ImageFormat::Jpeg,
            Self::Webp => ImageFormat::Webp,
            Self::Gif => ImageFormat::Gif,
            Self::Svg => ImageFormat::Svg,
            Self::Bmp => ImageFormat::Bmp,
            Self::Ico => ImageFormat::Ico,
        }
    }
}

struct IconData {
    format: StoredIconFormat,
    bytes: Vec<u8>,
}

struct RefreshRequest {
    path: PathBuf,
    identity: CacheIdentity,
}

#[derive(Default)]
struct FileIconState {
    initialized: bool,
    resolved: HashMap<FileIconKey, Arc<Image>>,
    refresh: Option<RefreshRequest>,
}

/// Holds only already-decoded type icons. It never performs native lookups from
/// a list render closure, so SFTP stays usable while disconnected.
#[derive(Clone, Default)]
pub(crate) struct FileIconCache(Rc<RefCell<FileIconState>>);

impl FileIconCache {
    /// Load and decode the persisted cache exactly once, when the SFTP UI first
    /// needs type icons. A default cache stays allocation-only until this call.
    pub(crate) fn load_if_needed(&self, path: Option<PathBuf>) -> bool {
        {
            let mut state = self.0.borrow_mut();
            if state.initialized {
                return false;
            }
            state.initialized = true;
        }

        let Some(path) = path else {
            return true;
        };
        let identity = cache_identity();
        let mut state = self.0.borrow_mut();
        if let Some(stored) = read_cache(&path, &identity) {
            state.resolved = stored.resolved_images();
        } else {
            state.refresh = Some(RefreshRequest { path, identity });
        }
        true
    }

    pub(crate) fn remote_icon(&self, name: &str, is_dir: bool) -> Option<Arc<Image>> {
        self.icon_for_key(icon_key_for_name(name, is_dir))
    }

    pub(crate) fn local_icon(&self, name: &str, is_dir: bool) -> Option<Arc<Image>> {
        self.icon_for_key(icon_key_for_name(name, is_dir))
    }

    fn icon_for_key(&self, key: FileIconKey) -> Option<Arc<Image>> {
        self.0.borrow().resolved.get(&key).cloned()
    }

    fn take_refresh_request(&self) -> Option<RefreshRequest> {
        self.0.borrow_mut().refresh.take()
    }

    fn replace_from_stored(&self, stored: &StoredFileIconCache) {
        self.0.borrow_mut().resolved = stored.resolved_images();
    }
}

impl crate::AxShell {
    pub(crate) fn start_file_icon_cache_refresh(&mut self, cx: &mut Context<Self>) {
        let Some(request) = self.file_icons.take_refresh_request() else {
            return;
        };
        let file_icons = self.file_icons.clone();

        #[cfg(target_os = "macos")]
        {
            // AppKit's icon APIs are main-thread APIs. Resolve a few entries per
            // UI turn, then write and reload the completed cache in the background.
            cx.spawn(async move |this, cx| {
                let mut stored = StoredFileIconCache::empty(&request.identity);
                for (index, key) in cache_keys().into_iter().enumerate() {
                    add_resolved_icon(&mut stored, key, &request.identity);
                    if (index + 1) % MACOS_ICONS_PER_TICK == 0 {
                        cx.background_executor()
                            .timer(Duration::from_millis(1))
                            .await;
                    }
                }
                let path = request.path;
                let identity = request.identity;
                let write_task = cx
                    .background_executor()
                    .spawn(async move { write_and_reload_cache(&path, &identity, &stored) });
                let refreshed = write_task.await;
                apply_refresh_result(&this, cx, &file_icons, refreshed);
            })
            .detach();
        }

        #[cfg(not(target_os = "macos"))]
        {
            let refresh_task = cx
                .background_executor()
                .spawn(async move { build_and_write_cache(&request) });
            cx.spawn(async move |this, cx| {
                let refreshed = refresh_task.await;
                apply_refresh_result(&this, cx, &file_icons, refreshed);
            })
            .detach();
        }
    }
}

fn apply_refresh_result(
    this: &gpui::WeakEntity<crate::AxShell>,
    cx: &mut gpui::AsyncApp,
    file_icons: &FileIconCache,
    refreshed: Result<StoredFileIconCache, String>,
) {
    match refreshed {
        Ok(stored) => {
            let cache = file_icons.clone();
            let _ = this.update(cx, move |_this, cx| {
                cache.replace_from_stored(&stored);
                cx.notify();
            });
        }
        Err(error) => tracing::warn!(
            component = "file_icons",
            operation = "refresh_cache",
            error = %crate::diagnostics::sanitize_error(&error),
            "Failed to refresh system file icon cache"
        ),
    }
}

fn icon_key_for_name(name: &str, is_dir: bool) -> FileIconKey {
    if is_dir {
        return FileIconKey::Directory;
    }

    let extension = Path::new(name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension {
        Some(extension) if COMMON_EXTENSIONS.contains(&extension.as_str()) => {
            FileIconKey::FileExtension(extension)
        }
        _ => FileIconKey::GenericFile,
    }
}

fn cache_keys() -> Vec<FileIconKey> {
    let mut keys = Vec::with_capacity(COMMON_EXTENSIONS.len() + 2);
    keys.push(FileIconKey::Directory);
    keys.push(FileIconKey::GenericFile);
    keys.extend(
        COMMON_EXTENSIONS
            .iter()
            .map(|extension| FileIconKey::FileExtension((*extension).to_string())),
    );
    keys
}

fn cache_identity() -> CacheIdentity {
    CacheIdentity {
        platform: std::env::consts::OS.to_string(),
        theme: current_theme_marker(),
    }
}

#[cfg(target_os = "linux")]
fn current_theme_marker() -> Option<String> {
    Some(freedesktop_icons::default_theme_gtk().unwrap_or_else(|| "hicolor".to_string()))
}

#[cfg(not(target_os = "linux"))]
fn current_theme_marker() -> Option<String> {
    None
}

fn read_cache(path: &Path, identity: &CacheIdentity) -> Option<StoredFileIconCache> {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return None,
        Err(error) => {
            tracing::warn!(
                component = "file_icons",
                operation = "read_cache",
                cache_path = %crate::diagnostics::mask_path(&path.to_string_lossy()),
                error = %crate::diagnostics::sanitize_error(&error.to_string()),
                "Failed to read system file icon cache"
            );
            return None;
        }
    };
    let stored = match serde_json::from_str::<StoredFileIconCache>(&raw) {
        Ok(stored) => stored,
        Err(error) => {
            tracing::warn!(
                component = "file_icons",
                operation = "parse_cache",
                cache_path = %crate::diagnostics::mask_path(&path.to_string_lossy()),
                error = %crate::diagnostics::sanitize_error(&error.to_string()),
                "Invalid system file icon cache; scheduling a refresh"
            );
            return None;
        }
    };
    if stored.is_compatible(identity) {
        Some(stored)
    } else {
        tracing::info!(
            component = "file_icons",
            operation = "validate_cache",
            cache_path = %crate::diagnostics::mask_path(&path.to_string_lossy()),
            "System file icon cache is stale; scheduling a refresh"
        );
        None
    }
}

#[cfg(not(target_os = "macos"))]
fn build_and_write_cache(request: &RefreshRequest) -> Result<StoredFileIconCache, String> {
    let mut stored = StoredFileIconCache::empty(&request.identity);
    for key in cache_keys() {
        add_resolved_icon(&mut stored, key, &request.identity);
    }
    write_and_reload_cache(&request.path, &request.identity, &stored)
}

fn add_resolved_icon(stored: &mut StoredFileIconCache, key: FileIconKey, identity: &CacheIdentity) {
    if let Some(icon) = resolve_system_icon(&key, identity) {
        stored
            .icons
            .insert(key.cache_key(), StoredIcon::from_icon_data(icon));
    }
}

fn write_and_reload_cache(
    path: &Path,
    identity: &CacheIdentity,
    stored: &StoredFileIconCache,
) -> Result<StoredFileIconCache, String> {
    write_cache_atomically(path, stored)?;
    read_cache(path, identity)
        .ok_or_else(|| "failed to reload the system file icon cache after writing it".to_string())
}

fn write_cache_atomically(path: &Path, stored: &StoredFileIconCache) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("cache path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let raw = serde_json::to_vec_pretty(stored).map_err(|error| error.to_string())?;
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, raw).map_err(|error| error.to_string())?;
    fs::File::open(&temporary_path)
        .and_then(|file| file.sync_all())
        .map_err(|error| error.to_string())?;
    replace_file_atomically(&temporary_path, path).map_err(|error| error.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(mut permissions) = fs::metadata(path).map(|metadata| metadata.permissions()) {
            permissions.set_mode(0o600);
            let _ = fs::set_permissions(path, permissions);
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn replace_file_atomically(temporary_path: &Path, path: &Path) -> io::Result<()> {
    fs::rename(temporary_path, path)
}

#[cfg(target_os = "windows")]
fn replace_file_atomically(temporary_path: &Path, path: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt as _;

    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW, ReplaceFileW,
    };

    let temporary = temporary_path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let destination = path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let replaced = unsafe {
        if path.exists() {
            ReplaceFileW(
                destination.as_ptr(),
                temporary.as_ptr(),
                std::ptr::null(),
                0,
                std::ptr::null(),
                std::ptr::null(),
            ) != 0
        } else {
            MoveFileExW(
                temporary.as_ptr(),
                destination.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            ) != 0
        }
    };
    if replaced {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(target_os = "macos")]
fn resolve_system_icon(key: &FileIconKey, _identity: &CacheIdentity) -> Option<IconData> {
    macos_system_icon(key)
}

#[cfg(target_os = "windows")]
fn resolve_system_icon(key: &FileIconKey, _identity: &CacheIdentity) -> Option<IconData> {
    windows_system_icon(key)
}

#[cfg(target_os = "linux")]
fn resolve_system_icon(key: &FileIconKey, identity: &CacheIdentity) -> Option<IconData> {
    linux_system_icon(key, identity.theme.as_deref().unwrap_or("hicolor"))
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn resolve_system_icon(_key: &FileIconKey, _identity: &CacheIdentity) -> Option<IconData> {
    None
}

#[cfg(target_os = "macos")]
fn macos_system_icon(key: &FileIconKey) -> Option<IconData> {
    use std::{ffi::CString, slice};

    use objc::{class, msg_send, rc::autoreleasepool, runtime::Object, sel, sel_impl};

    autoreleasepool(|| unsafe {
        let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        let (value, use_file_type) = match key {
            FileIconKey::Directory => ("/tmp", false),
            FileIconKey::GenericFile => ("", true),
            FileIconKey::FileExtension(extension) => (extension.as_str(), true),
        };
        let value = CString::new(value).ok()?;
        let string: *mut Object = msg_send![class!(NSString), stringWithUTF8String: value.as_ptr()];
        if string.is_null() {
            return None;
        }
        let icon: *mut Object = if use_file_type {
            msg_send![workspace, iconForFileType: string]
        } else {
            msg_send![workspace, iconForFile: string]
        };
        if icon.is_null() {
            return None;
        }

        let tiff_data: *mut Object = msg_send![icon, TIFFRepresentation];
        if tiff_data.is_null() {
            return None;
        }
        let bitmap_rep: *mut Object =
            msg_send![class!(NSBitmapImageRep), imageRepWithData: tiff_data];
        if bitmap_rep.is_null() {
            return None;
        }
        // NSPNGFileType is 4. Persisting PNG keeps the cache platform-neutral.
        let png_data: *mut Object = msg_send![bitmap_rep,
            representationUsingType: 4usize
            properties: std::ptr::null_mut::<Object>()
        ];
        if png_data.is_null() {
            return None;
        }
        let length: usize = msg_send![png_data, length];
        let bytes: *const u8 = msg_send![png_data, bytes];
        if bytes.is_null() || length == 0 {
            return None;
        }
        Some(IconData {
            format: StoredIconFormat::Png,
            bytes: slice::from_raw_parts(bytes, length).to_vec(),
        })
    })
}

#[cfg(target_os = "windows")]
fn windows_system_icon(key: &FileIconKey) -> Option<IconData> {
    use windows_sys::Win32::{
        Foundation::RPC_E_CHANGED_MODE,
        System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize},
    };

    let result = unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED as u32) };
    if result < 0 && result != RPC_E_CHANGED_MODE {
        return None;
    }
    let icon = windows_system_icon_inner(key);
    if result >= 0 {
        unsafe { CoUninitialize() };
    }
    icon
}

#[cfg(target_os = "windows")]
fn windows_system_icon_inner(key: &FileIconKey) -> Option<IconData> {
    use std::{ffi::c_void, io::Cursor, mem::size_of, ptr};

    use windows_sys::Win32::{
        Graphics::Gdi::{
            BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection,
            DIB_RGB_COLORS, DeleteDC, DeleteObject, HGDIOBJ, SelectObject,
        },
        Storage::FileSystem::{FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL},
        UI::{
            Shell::{
                SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON, SHGFI_USEFILEATTRIBUTES, SHGetFileInfoW,
            },
            WindowsAndMessaging::{DI_NORMAL, DestroyIcon, DrawIconEx},
        },
    };

    let (path, attributes) = match key {
        FileIconKey::Directory => ("folder".to_string(), FILE_ATTRIBUTE_DIRECTORY),
        FileIconKey::GenericFile => ("remote".to_string(), FILE_ATTRIBUTE_NORMAL),
        FileIconKey::FileExtension(extension) => {
            (format!("remote.{extension}"), FILE_ATTRIBUTE_NORMAL)
        }
    };
    let mut wide_path = path.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
    let mut file_info = unsafe { std::mem::zeroed::<SHFILEINFOW>() };
    let flags = SHGFI_ICON | SHGFI_SMALLICON | SHGFI_USEFILEATTRIBUTES;
    if unsafe {
        SHGetFileInfoW(
            wide_path.as_mut_ptr(),
            attributes,
            &mut file_info,
            size_of::<SHFILEINFOW>() as u32,
            flags,
        )
    } == 0
    {
        return None;
    }

    let dc = unsafe { CreateCompatibleDC(ptr::null_mut()) };
    if dc.is_null() {
        unsafe { DestroyIcon(file_info.hIcon) };
        return None;
    }
    let bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: ICON_SIZE.into(),
            biHeight: -(i32::from(ICON_SIZE)),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut bits = ptr::null_mut::<c_void>();
    let bitmap = unsafe {
        CreateDIBSection(
            dc,
            &bitmap_info,
            DIB_RGB_COLORS,
            &mut bits,
            ptr::null_mut(),
            0,
        )
    };
    if bitmap.is_null() || bits.is_null() {
        unsafe {
            if !bitmap.is_null() {
                DeleteObject(bitmap as HGDIOBJ);
            }
            DeleteDC(dc);
            DestroyIcon(file_info.hIcon);
        }
        return None;
    }

    let old_object = unsafe { SelectObject(dc, bitmap as HGDIOBJ) };
    let drawn = unsafe {
        DrawIconEx(
            dc,
            0,
            0,
            file_info.hIcon,
            ICON_SIZE.into(),
            ICON_SIZE.into(),
            0,
            ptr::null_mut(),
            DI_NORMAL,
        ) != 0
    };
    let byte_count = usize::from(ICON_SIZE) * usize::from(ICON_SIZE) * 4;
    let mut pixels = if drawn {
        unsafe { std::slice::from_raw_parts(bits.cast::<u8>(), byte_count).to_vec() }
    } else {
        Vec::new()
    };
    unsafe {
        if !old_object.is_null() {
            SelectObject(dc, old_object);
        }
        DeleteObject(bitmap as HGDIOBJ);
        DeleteDC(dc);
        DestroyIcon(file_info.hIcon);
    }
    if pixels.is_empty() {
        return None;
    }
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2); // Windows DIBs are BGRA; PNG expects RGBA.
    }
    let image = image::RgbaImage::from_raw(ICON_SIZE.into(), ICON_SIZE.into(), pixels)?;
    let mut png = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image)
        .write_to(&mut png, image::ImageFormat::Png)
        .ok()?;
    Some(IconData {
        format: StoredIconFormat::Png,
        bytes: png.into_inner(),
    })
}

#[cfg(target_os = "linux")]
fn linux_system_icon(key: &FileIconKey, theme: &str) -> Option<IconData> {
    use freedesktop_icons::lookup;

    linux_icon_names(key)
        .into_iter()
        .find_map(|name| {
            lookup(&name)
                .with_theme(theme)
                .with_size(ICON_SIZE)
                .with_cache()
                .find()
        })
        .and_then(|path| icon_data_from_path(&path))
}

#[cfg(target_os = "linux")]
fn linux_icon_names(key: &FileIconKey) -> Vec<String> {
    match key {
        FileIconKey::Directory => vec!["folder".to_string(), "inode-directory".to_string()],
        FileIconKey::GenericFile => vec![
            "application-octet-stream".to_string(),
            "text-x-generic".to_string(),
            "unknown".to_string(),
        ],
        FileIconKey::FileExtension(extension) => {
            let mime = mime_guess::from_ext(extension)
                .first_or_octet_stream()
                .essence_str()
                .to_string();
            let mut names = vec![mime.replace('/', "-")];
            if let Some((category, _)) = mime.split_once('/') {
                names.push(format!("{category}-x-generic"));
            }
            names.extend(["text-x-generic".to_string(), "unknown".to_string()]);
            names
        }
    }
}

#[cfg(target_os = "linux")]
fn icon_data_from_path(path: &Path) -> Option<IconData> {
    let format = match path.extension().and_then(|extension| extension.to_str())? {
        extension if extension.eq_ignore_ascii_case("png") => StoredIconFormat::Png,
        extension if extension.eq_ignore_ascii_case("svg") => StoredIconFormat::Svg,
        extension
            if extension.eq_ignore_ascii_case("jpg") || extension.eq_ignore_ascii_case("jpeg") =>
        {
            StoredIconFormat::Jpeg
        }
        extension if extension.eq_ignore_ascii_case("webp") => StoredIconFormat::Webp,
        extension if extension.eq_ignore_ascii_case("gif") => StoredIconFormat::Gif,
        extension if extension.eq_ignore_ascii_case("bmp") => StoredIconFormat::Bmp,
        extension if extension.eq_ignore_ascii_case("ico") => StoredIconFormat::Ico,
        _ => return None,
    };
    Some(IconData {
        format,
        bytes: fs::read(path).ok()?,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CACHE_VERSION, CacheIdentity, FileIconCache, FileIconKey, StoredFileIconCache,
        icon_key_for_name, read_cache, write_cache_atomically,
    };
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn files_with_the_same_extension_share_a_cache_key() {
        assert_eq!(
            icon_key_for_name("README.TXT", false),
            icon_key_for_name("notes.txt", false)
        );
    }

    #[test]
    fn directories_share_one_cache_key() {
        assert_eq!(
            icon_key_for_name("src", true),
            icon_key_for_name("assets", true)
        );
        assert_eq!(icon_key_for_name("src", true), FileIconKey::Directory);
    }

    #[test]
    fn unsupported_extensions_use_the_generic_file_icon() {
        assert_eq!(
            icon_key_for_name("opaque.custom-format", false),
            FileIconKey::GenericFile
        );
    }

    #[test]
    fn default_cache_initializes_only_for_the_first_sftp_request() {
        let cache = FileIconCache::default();
        assert!(!cache.0.borrow().initialized);

        assert!(cache.load_if_needed(None));
        assert!(cache.0.borrow().initialized);
        assert!(!cache.load_if_needed(None));
    }

    #[test]
    fn stored_cache_requires_matching_identity_and_key_set() {
        let identity = CacheIdentity {
            platform: "test".to_string(),
            theme: Some("theme".to_string()),
        };
        let mut stored = StoredFileIconCache::empty(&identity);
        assert!(stored.is_compatible(&identity));

        stored.version = CACHE_VERSION + 1;
        assert!(!stored.is_compatible(&identity));
    }

    #[test]
    fn cache_file_round_trips_after_an_atomic_write() {
        let identity = CacheIdentity {
            platform: "test".to_string(),
            theme: None,
        };
        let stored = StoredFileIconCache::empty(&identity);
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("ax-shell-file-icons-{unique}"));
        let path = dir.join("file-icons.json");

        write_cache_atomically(&path, &stored).expect("cache should write atomically");
        assert_eq!(read_cache(&path, &identity).unwrap().version, CACHE_VERSION);

        fs::remove_dir_all(dir).expect("temporary cache directory should be removable");
    }
}
