// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod action;
pub use self::action::ActionExt;
pub use self::action::{Action, NONE_ACTION};

mod action_group;
pub use self::action_group::ActionGroupExt;
pub use self::action_group::{ActionGroup, NONE_ACTION_GROUP};

mod action_map;
pub use self::action_map::ActionMapExt;
pub use self::action_map::{ActionMap, NONE_ACTION_MAP};

mod app_info;
pub use self::app_info::AppInfoExt;
pub use self::app_info::{AppInfo, NONE_APP_INFO};

mod app_info_monitor;
pub use self::app_info_monitor::AppInfoMonitor;

mod app_launch_context;
pub use self::app_launch_context::AppLaunchContextExt;
pub use self::app_launch_context::{AppLaunchContext, NONE_APP_LAUNCH_CONTEXT};

mod application;
pub use self::application::ApplicationBuilder;
pub use self::application::ApplicationExt;
pub use self::application::{Application, NONE_APPLICATION};

mod application_command_line;
pub use self::application_command_line::ApplicationCommandLineExt;
pub use self::application_command_line::{ApplicationCommandLine, NONE_APPLICATION_COMMAND_LINE};

mod buffered_input_stream;
pub use self::buffered_input_stream::BufferedInputStreamBuilder;
pub use self::buffered_input_stream::BufferedInputStreamExt;
pub use self::buffered_input_stream::{BufferedInputStream, NONE_BUFFERED_INPUT_STREAM};

mod buffered_output_stream;
pub use self::buffered_output_stream::BufferedOutputStreamBuilder;
pub use self::buffered_output_stream::BufferedOutputStreamExt;
pub use self::buffered_output_stream::{BufferedOutputStream, NONE_BUFFERED_OUTPUT_STREAM};

mod bytes_icon;
pub use self::bytes_icon::BytesIcon;

mod cancellable;
pub use self::cancellable::CancellableExt;
pub use self::cancellable::{Cancellable, NONE_CANCELLABLE};

mod charset_converter;
pub use self::charset_converter::CharsetConverterBuilder;
pub use self::charset_converter::CharsetConverterExt;
pub use self::charset_converter::{CharsetConverter, NONE_CHARSET_CONVERTER};

mod converter;
pub use self::converter::ConverterExt;
pub use self::converter::{Converter, NONE_CONVERTER};

mod converter_input_stream;
pub use self::converter_input_stream::ConverterInputStreamBuilder;
pub use self::converter_input_stream::ConverterInputStreamExt;
pub use self::converter_input_stream::{ConverterInputStream, NONE_CONVERTER_INPUT_STREAM};

mod converter_output_stream;
pub use self::converter_output_stream::ConverterOutputStreamBuilder;
pub use self::converter_output_stream::ConverterOutputStreamExt;
pub use self::converter_output_stream::{ConverterOutputStream, NONE_CONVERTER_OUTPUT_STREAM};

mod credentials;
pub use self::credentials::Credentials;

mod dbus_auth_observer;
pub use self::dbus_auth_observer::DBusAuthObserver;

mod dbus_connection;
pub use self::dbus_connection::DBusConnection;

mod dbus_interface;
pub use self::dbus_interface::DBusInterfaceExt;
pub use self::dbus_interface::{DBusInterface, NONE_DBUS_INTERFACE};

mod dbus_interface_skeleton;
pub use self::dbus_interface_skeleton::DBusInterfaceSkeletonExt;
pub use self::dbus_interface_skeleton::{DBusInterfaceSkeleton, NONE_DBUS_INTERFACE_SKELETON};

mod dbus_menu_model;
pub use self::dbus_menu_model::DBusMenuModel;

mod dbus_message;
pub use self::dbus_message::DBusMessage;

mod dbus_method_invocation;
pub use self::dbus_method_invocation::DBusMethodInvocation;

mod dbus_object;
pub use self::dbus_object::DBusObjectExt;
pub use self::dbus_object::{DBusObject, NONE_DBUS_OBJECT};

mod dbus_proxy;
pub use self::dbus_proxy::DBusProxyExt;
pub use self::dbus_proxy::{DBusProxy, NONE_DBUS_PROXY};

mod dbus_server;
pub use self::dbus_server::DBusServer;

mod data_input_stream;
pub use self::data_input_stream::DataInputStreamBuilder;
pub use self::data_input_stream::DataInputStreamExt;
pub use self::data_input_stream::{DataInputStream, NONE_DATA_INPUT_STREAM};

mod data_output_stream;
pub use self::data_output_stream::DataOutputStreamBuilder;
pub use self::data_output_stream::DataOutputStreamExt;
pub use self::data_output_stream::{DataOutputStream, NONE_DATA_OUTPUT_STREAM};

#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(not(windows), not(target_os = "macos")))))]
mod desktop_app_info;
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(not(windows), not(target_os = "macos")))))]
pub use self::desktop_app_info::DesktopAppInfoExt;
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(not(windows), not(target_os = "macos")))))]
pub use self::desktop_app_info::{DesktopAppInfo, NONE_DESKTOP_APP_INFO};

mod drive;
pub use self::drive::DriveExt;
pub use self::drive::{Drive, NONE_DRIVE};

mod emblem;
pub use self::emblem::Emblem;

mod emblemed_icon;
pub use self::emblemed_icon::EmblemedIconExt;
pub use self::emblemed_icon::{EmblemedIcon, NONE_EMBLEMED_ICON};

mod file;
pub use self::file::FileExt;
pub use self::file::{File, NONE_FILE};

mod file_enumerator;
pub use self::file_enumerator::FileEnumeratorExt;
pub use self::file_enumerator::{FileEnumerator, NONE_FILE_ENUMERATOR};

mod file_io_stream;
pub use self::file_io_stream::FileIOStreamExt;
pub use self::file_io_stream::{FileIOStream, NONE_FILE_IO_STREAM};

mod file_icon;
pub use self::file_icon::FileIcon;

mod file_info;
pub use self::file_info::FileInfo;

mod file_input_stream;
pub use self::file_input_stream::FileInputStreamExt;
pub use self::file_input_stream::{FileInputStream, NONE_FILE_INPUT_STREAM};

mod file_monitor;
pub use self::file_monitor::FileMonitorExt;
pub use self::file_monitor::{FileMonitor, NONE_FILE_MONITOR};

mod file_output_stream;
pub use self::file_output_stream::FileOutputStreamExt;
pub use self::file_output_stream::{FileOutputStream, NONE_FILE_OUTPUT_STREAM};

mod filename_completer;
pub use self::filename_completer::FilenameCompleterExt;
pub use self::filename_completer::{FilenameCompleter, NONE_FILENAME_COMPLETER};

mod filter_input_stream;
pub use self::filter_input_stream::FilterInputStreamExt;
pub use self::filter_input_stream::{FilterInputStream, NONE_FILTER_INPUT_STREAM};

mod filter_output_stream;
pub use self::filter_output_stream::FilterOutputStreamExt;
pub use self::filter_output_stream::{FilterOutputStream, NONE_FILTER_OUTPUT_STREAM};

mod io_stream;
pub use self::io_stream::IOStreamExt;
pub use self::io_stream::{IOStream, NONE_IO_STREAM};

mod icon;
pub use self::icon::IconExt;
pub use self::icon::{Icon, NONE_ICON};

mod inet_address;
pub use self::inet_address::InetAddressExt;
pub use self::inet_address::{InetAddress, NONE_INET_ADDRESS};

mod inet_address_mask;
pub use self::inet_address_mask::InetAddressMaskExt;
pub use self::inet_address_mask::{InetAddressMask, NONE_INET_ADDRESS_MASK};

mod inet_socket_address;
pub use self::inet_socket_address::InetSocketAddressExt;
pub use self::inet_socket_address::{InetSocketAddress, NONE_INET_SOCKET_ADDRESS};

mod input_stream;
pub use self::input_stream::InputStreamExt;
pub use self::input_stream::{InputStream, NONE_INPUT_STREAM};

#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
mod list_model;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::list_model::ListModelExt;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::list_model::{ListModel, NONE_LIST_MODEL};

#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
mod list_store;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::list_store::ListStoreBuilder;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::list_store::ListStoreExt;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::list_store::{ListStore, NONE_LIST_STORE};

mod loadable_icon;
pub use self::loadable_icon::LoadableIconExt;
pub use self::loadable_icon::{LoadableIcon, NONE_LOADABLE_ICON};

mod memory_input_stream;
pub use self::memory_input_stream::MemoryInputStreamExt;
pub use self::memory_input_stream::{MemoryInputStream, NONE_MEMORY_INPUT_STREAM};

#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
mod memory_monitor;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
pub use self::memory_monitor::MemoryMonitorExt;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
pub use self::memory_monitor::{MemoryMonitor, NONE_MEMORY_MONITOR};

mod memory_output_stream;
pub use self::memory_output_stream::MemoryOutputStreamExt;
pub use self::memory_output_stream::{MemoryOutputStream, NONE_MEMORY_OUTPUT_STREAM};

mod menu;
pub use self::menu::Menu;

mod menu_attribute_iter;
pub use self::menu_attribute_iter::MenuAttributeIterExt;
pub use self::menu_attribute_iter::{MenuAttributeIter, NONE_MENU_ATTRIBUTE_ITER};

mod menu_item;
pub use self::menu_item::MenuItem;

mod menu_link_iter;
pub use self::menu_link_iter::MenuLinkIterExt;
pub use self::menu_link_iter::{MenuLinkIter, NONE_MENU_LINK_ITER};

mod menu_model;
pub use self::menu_model::MenuModelExt;
pub use self::menu_model::{MenuModel, NONE_MENU_MODEL};

mod mount;
pub use self::mount::MountExt;
pub use self::mount::{Mount, NONE_MOUNT};

mod mount_operation;
pub use self::mount_operation::MountOperationExt;
pub use self::mount_operation::{MountOperation, NONE_MOUNT_OPERATION};

mod network_address;
pub use self::network_address::NetworkAddressExt;
pub use self::network_address::{NetworkAddress, NONE_NETWORK_ADDRESS};

mod network_monitor;
pub use self::network_monitor::NetworkMonitorExt;
pub use self::network_monitor::{NetworkMonitor, NONE_NETWORK_MONITOR};

mod network_service;
pub use self::network_service::NetworkServiceExt;
pub use self::network_service::{NetworkService, NONE_NETWORK_SERVICE};

mod notification;
pub use self::notification::Notification;

mod output_stream;
pub use self::output_stream::OutputStreamExt;
pub use self::output_stream::{OutputStream, NONE_OUTPUT_STREAM};

mod permission;
pub use self::permission::PermissionExt;
pub use self::permission::{Permission, NONE_PERMISSION};

mod pollable_input_stream;
pub use self::pollable_input_stream::PollableInputStreamExt;
pub use self::pollable_input_stream::{PollableInputStream, NONE_POLLABLE_INPUT_STREAM};

mod pollable_output_stream;
pub use self::pollable_output_stream::PollableOutputStreamExt;
pub use self::pollable_output_stream::{PollableOutputStream, NONE_POLLABLE_OUTPUT_STREAM};

mod property_action;
pub use self::property_action::PropertyAction;

mod proxy;
pub use self::proxy::ProxyExt;
pub use self::proxy::{Proxy, NONE_PROXY};

mod proxy_address;
pub use self::proxy_address::ProxyAddressExt;
pub use self::proxy_address::{ProxyAddress, NONE_PROXY_ADDRESS};

mod proxy_resolver;
pub use self::proxy_resolver::ProxyResolverExt;
pub use self::proxy_resolver::{ProxyResolver, NONE_PROXY_RESOLVER};

mod remote_action_group;
pub use self::remote_action_group::RemoteActionGroupExt;
pub use self::remote_action_group::{RemoteActionGroup, NONE_REMOTE_ACTION_GROUP};

mod resolver;
pub use self::resolver::ResolverExt;
pub use self::resolver::{Resolver, NONE_RESOLVER};

mod seekable;
pub use self::seekable::SeekableExt;
pub use self::seekable::{Seekable, NONE_SEEKABLE};

mod settings;
pub use self::settings::SettingsExt;
pub use self::settings::{Settings, NONE_SETTINGS};

mod settings_backend;
pub use self::settings_backend::SettingsBackendExt;
pub use self::settings_backend::{SettingsBackend, NONE_SETTINGS_BACKEND};

mod simple_action;
pub use self::simple_action::SimpleAction;

mod simple_action_group;
pub use self::simple_action_group::{SimpleActionGroup, NONE_SIMPLE_ACTION_GROUP};

#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
mod simple_io_stream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::simple_io_stream::SimpleIOStream;

mod simple_permission;
pub use self::simple_permission::SimplePermission;

mod socket;
pub use self::socket::SocketExt;
pub use self::socket::{Socket, NONE_SOCKET};

mod socket_address;
pub use self::socket_address::SocketAddressExt;
pub use self::socket_address::{SocketAddress, NONE_SOCKET_ADDRESS};

mod socket_address_enumerator;
pub use self::socket_address_enumerator::SocketAddressEnumeratorExt;
pub use self::socket_address_enumerator::{
    SocketAddressEnumerator, NONE_SOCKET_ADDRESS_ENUMERATOR,
};

mod socket_client;
pub use self::socket_client::SocketClientExt;
pub use self::socket_client::{SocketClient, NONE_SOCKET_CLIENT};

mod socket_connectable;
pub use self::socket_connectable::SocketConnectableExt;
pub use self::socket_connectable::{SocketConnectable, NONE_SOCKET_CONNECTABLE};

mod socket_connection;
pub use self::socket_connection::SocketConnectionExt;
pub use self::socket_connection::{SocketConnection, NONE_SOCKET_CONNECTION};

mod socket_listener;
pub use self::socket_listener::SocketListenerExt;
pub use self::socket_listener::{SocketListener, NONE_SOCKET_LISTENER};

mod socket_service;
pub use self::socket_service::SocketServiceExt;
pub use self::socket_service::{SocketService, NONE_SOCKET_SERVICE};

mod subprocess;
pub use self::subprocess::Subprocess;

mod subprocess_launcher;
pub use self::subprocess_launcher::SubprocessLauncher;

mod tcp_connection;
pub use self::tcp_connection::TcpConnectionExt;
pub use self::tcp_connection::{TcpConnection, NONE_TCP_CONNECTION};

mod themed_icon;
pub use self::themed_icon::ThemedIcon;

mod threaded_socket_service;
pub use self::threaded_socket_service::ThreadedSocketServiceExt;
pub use self::threaded_socket_service::{ThreadedSocketService, NONE_THREADED_SOCKET_SERVICE};

mod tls_backend;
pub use self::tls_backend::TlsBackendExt;
pub use self::tls_backend::{TlsBackend, NONE_TLS_BACKEND};

mod tls_certificate;
pub use self::tls_certificate::TlsCertificateExt;
pub use self::tls_certificate::{TlsCertificate, NONE_TLS_CERTIFICATE};

mod tls_client_connection;
pub use self::tls_client_connection::TlsClientConnectionExt;
pub use self::tls_client_connection::{TlsClientConnection, NONE_TLS_CLIENT_CONNECTION};

mod tls_connection;
pub use self::tls_connection::TlsConnectionExt;
pub use self::tls_connection::{TlsConnection, NONE_TLS_CONNECTION};

mod tls_database;
pub use self::tls_database::TlsDatabaseExt;
pub use self::tls_database::{TlsDatabase, NONE_TLS_DATABASE};

mod tls_file_database;
pub use self::tls_file_database::TlsFileDatabaseExt;
pub use self::tls_file_database::{TlsFileDatabase, NONE_TLS_FILE_DATABASE};

mod tls_interaction;
pub use self::tls_interaction::TlsInteractionExt;
pub use self::tls_interaction::{TlsInteraction, NONE_TLS_INTERACTION};

mod tls_password;
pub use self::tls_password::TlsPasswordExt;
pub use self::tls_password::{TlsPassword, NONE_TLS_PASSWORD};

mod tls_server_connection;
pub use self::tls_server_connection::TlsServerConnectionExt;
pub use self::tls_server_connection::{TlsServerConnection, NONE_TLS_SERVER_CONNECTION};

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
mod unix_fd_list;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_fd_list::UnixFDListExt;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_fd_list::{UnixFDList, NONE_UNIX_FD_LIST};

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
mod unix_input_stream;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_input_stream::UnixInputStreamExt;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_input_stream::{UnixInputStream, NONE_UNIX_INPUT_STREAM};

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
mod unix_output_stream;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_output_stream::UnixOutputStreamExt;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_output_stream::{UnixOutputStream, NONE_UNIX_OUTPUT_STREAM};

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
mod unix_socket_address;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_socket_address::UnixSocketAddressExt;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
pub use self::unix_socket_address::{UnixSocketAddress, NONE_UNIX_SOCKET_ADDRESS};

mod vfs;
pub use self::vfs::VfsExt;
pub use self::vfs::{Vfs, NONE_VFS};

mod volume;
pub use self::volume::VolumeExt;
pub use self::volume::{Volume, NONE_VOLUME};

mod volume_monitor;
pub use self::volume_monitor::VolumeMonitorExt;
pub use self::volume_monitor::{VolumeMonitor, NONE_VOLUME_MONITOR};

mod zlib_compressor;
pub use self::zlib_compressor::ZlibCompressorExt;
pub use self::zlib_compressor::{ZlibCompressor, NONE_ZLIB_COMPRESSOR};

mod zlib_decompressor;
pub use self::zlib_decompressor::ZlibDecompressorExt;
pub use self::zlib_decompressor::{ZlibDecompressor, NONE_ZLIB_DECOMPRESSOR};

mod dbus_arg_info;
pub use self::dbus_arg_info::DBusArgInfo;

mod dbus_interface_info;
pub use self::dbus_interface_info::DBusInterfaceInfo;

mod dbus_method_info;
pub use self::dbus_method_info::DBusMethodInfo;

mod dbus_node_info;
pub use self::dbus_node_info::DBusNodeInfo;

mod dbus_property_info;
pub use self::dbus_property_info::DBusPropertyInfo;

mod dbus_signal_info;
pub use self::dbus_signal_info::DBusSignalInfo;

mod file_attribute_matcher;
pub use self::file_attribute_matcher::FileAttributeMatcher;

mod resource;
pub use self::resource::Resource;

mod settings_schema;
pub use self::settings_schema::SettingsSchema;

mod settings_schema_key;
pub use self::settings_schema_key::SettingsSchemaKey;

mod settings_schema_source;
pub use self::settings_schema_source::SettingsSchemaSource;

mod srv_target;
pub use self::srv_target::SrvTarget;

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
mod unix_mount_entry;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
pub use self::unix_mount_entry::UnixMountEntry;

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
mod unix_mount_point;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
pub use self::unix_mount_point::UnixMountPoint;

mod enums;
pub use self::enums::BusType;
pub use self::enums::ConverterResult;
pub use self::enums::CredentialsType;
pub use self::enums::DBusMessageByteOrder;
pub use self::enums::DBusMessageHeaderField;
pub use self::enums::DBusMessageType;
pub use self::enums::DataStreamByteOrder;
pub use self::enums::DataStreamNewlineType;
pub use self::enums::DriveStartStopType;
pub use self::enums::EmblemOrigin;
pub use self::enums::FileAttributeStatus;
pub use self::enums::FileAttributeType;
pub use self::enums::FileMonitorEvent;
pub use self::enums::FileType;
pub use self::enums::IOErrorEnum;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
pub use self::enums::MemoryMonitorWarningLevel;
pub use self::enums::MountOperationResult;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
pub use self::enums::NetworkConnectivity;
pub use self::enums::NotificationPriority;
pub use self::enums::PasswordSave;
pub use self::enums::ResolverRecordType;
pub use self::enums::ResourceError;
pub use self::enums::SocketClientEvent;
pub use self::enums::SocketFamily;
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
pub use self::enums::SocketListenerEvent;
pub use self::enums::SocketProtocol;
pub use self::enums::SocketType;
pub use self::enums::TlsAuthenticationMode;
pub use self::enums::TlsCertificateRequestFlags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::enums::TlsChannelBindingType;
pub use self::enums::TlsDatabaseLookupFlags;
pub use self::enums::TlsInteractionResult;
pub use self::enums::TlsRehandshakeMode;
pub use self::enums::UnixSocketAddressType;
pub use self::enums::ZlibCompressorFormat;

mod flags;
pub use self::flags::AppInfoCreateFlags;
pub use self::flags::ApplicationFlags;
pub use self::flags::AskPasswordFlags;
pub use self::flags::BusNameOwnerFlags;
pub use self::flags::BusNameWatcherFlags;
pub use self::flags::ConverterFlags;
pub use self::flags::DBusCallFlags;
pub use self::flags::DBusCapabilityFlags;
pub use self::flags::DBusConnectionFlags;
pub use self::flags::DBusInterfaceSkeletonFlags;
pub use self::flags::DBusMessageFlags;
pub use self::flags::DBusProxyFlags;
pub use self::flags::DBusSendMessageFlags;
pub use self::flags::DBusServerFlags;
pub use self::flags::DBusSignalFlags;
pub use self::flags::DriveStartFlags;
pub use self::flags::FileCopyFlags;
pub use self::flags::FileCreateFlags;
pub use self::flags::FileMeasureFlags;
pub use self::flags::FileMonitorFlags;
pub use self::flags::FileQueryInfoFlags;
pub use self::flags::IOStreamSpliceFlags;
pub use self::flags::MountMountFlags;
pub use self::flags::MountUnmountFlags;
pub use self::flags::OutputStreamSpliceFlags;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
pub use self::flags::ResolverNameLookupFlags;
pub use self::flags::ResourceLookupFlags;
pub use self::flags::SettingsBindFlags;
pub use self::flags::SubprocessFlags;
pub use self::flags::TlsCertificateFlags;
pub use self::flags::TlsDatabaseVerifyFlags;
pub use self::flags::TlsPasswordFlags;

pub mod functions;

mod constants;
#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
pub use self::constants::DRIVE_IDENTIFIER_KIND_UNIX_DEVICE;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_DELETE;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_READ;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_RENAME;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_TRASH;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_WRITE;
pub use self::constants::FILE_ATTRIBUTE_DOS_IS_ARCHIVE;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
pub use self::constants::FILE_ATTRIBUTE_DOS_IS_MOUNTPOINT;
pub use self::constants::FILE_ATTRIBUTE_DOS_IS_SYSTEM;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
pub use self::constants::FILE_ATTRIBUTE_DOS_REPARSE_POINT_TAG;
pub use self::constants::FILE_ATTRIBUTE_ETAG_VALUE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_FREE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_READONLY;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_REMOTE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_SIZE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_TYPE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_USED;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_USE_PREVIEW;
pub use self::constants::FILE_ATTRIBUTE_GVFS_BACKEND;
pub use self::constants::FILE_ATTRIBUTE_ID_FILE;
pub use self::constants::FILE_ATTRIBUTE_ID_FILESYSTEM;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_EJECT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_MOUNT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_POLL;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_START;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_START_DEGRADED;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_STOP;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_UNMOUNT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_HAL_UDI;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_IS_MEDIA_CHECK_AUTOMATIC;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_START_STOP_TYPE;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE_FILE;
pub use self::constants::FILE_ATTRIBUTE_OWNER_GROUP;
pub use self::constants::FILE_ATTRIBUTE_OWNER_USER;
pub use self::constants::FILE_ATTRIBUTE_OWNER_USER_REAL;
pub use self::constants::FILE_ATTRIBUTE_PREVIEW_ICON;
#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
pub use self::constants::FILE_ATTRIBUTE_RECENT_MODIFIED;
pub use self::constants::FILE_ATTRIBUTE_SELINUX_CONTEXT;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_ALLOCATED_SIZE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_COPY_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_DESCRIPTION;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_EDIT_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_ICON;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_BACKUP;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_HIDDEN;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_SYMLINK;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_VIRTUAL;
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_VOLATILE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SIZE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SORT_ORDER;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_TARGET_URI;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_TYPE;
pub use self::constants::FILE_ATTRIBUTE_THUMBNAILING_FAILED;
pub use self::constants::FILE_ATTRIBUTE_THUMBNAIL_IS_VALID;
pub use self::constants::FILE_ATTRIBUTE_THUMBNAIL_PATH;
pub use self::constants::FILE_ATTRIBUTE_TIME_ACCESS;
pub use self::constants::FILE_ATTRIBUTE_TIME_ACCESS_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_CHANGED;
pub use self::constants::FILE_ATTRIBUTE_TIME_CHANGED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_CREATED;
pub use self::constants::FILE_ATTRIBUTE_TIME_CREATED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_MODIFIED;
pub use self::constants::FILE_ATTRIBUTE_TIME_MODIFIED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TRASH_DELETION_DATE;
pub use self::constants::FILE_ATTRIBUTE_TRASH_ITEM_COUNT;
pub use self::constants::FILE_ATTRIBUTE_TRASH_ORIG_PATH;
pub use self::constants::FILE_ATTRIBUTE_UNIX_BLOCKS;
pub use self::constants::FILE_ATTRIBUTE_UNIX_BLOCK_SIZE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_DEVICE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_GID;
pub use self::constants::FILE_ATTRIBUTE_UNIX_INODE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_IS_MOUNTPOINT;
pub use self::constants::FILE_ATTRIBUTE_UNIX_MODE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_NLINK;
pub use self::constants::FILE_ATTRIBUTE_UNIX_RDEV;
pub use self::constants::FILE_ATTRIBUTE_UNIX_UID;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
pub use self::constants::MEMORY_MONITOR_EXTENSION_POINT_NAME;
pub use self::constants::MENU_ATTRIBUTE_ACTION;
pub use self::constants::MENU_ATTRIBUTE_ACTION_NAMESPACE;
pub use self::constants::MENU_ATTRIBUTE_ICON;
pub use self::constants::MENU_ATTRIBUTE_LABEL;
pub use self::constants::MENU_ATTRIBUTE_TARGET;
pub use self::constants::MENU_LINK_SECTION;
pub use self::constants::MENU_LINK_SUBMENU;
pub use self::constants::NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME;
pub use self::constants::NETWORK_MONITOR_EXTENSION_POINT_NAME;
pub use self::constants::PROXY_EXTENSION_POINT_NAME;
pub use self::constants::PROXY_RESOLVER_EXTENSION_POINT_NAME;
pub use self::constants::SETTINGS_BACKEND_EXTENSION_POINT_NAME;
pub use self::constants::TLS_BACKEND_EXTENSION_POINT_NAME;
pub use self::constants::TLS_DATABASE_PURPOSE_AUTHENTICATE_CLIENT;
pub use self::constants::TLS_DATABASE_PURPOSE_AUTHENTICATE_SERVER;
pub use self::constants::VFS_EXTENSION_POINT_NAME;
pub use self::constants::VOLUME_IDENTIFIER_KIND_CLASS;
pub use self::constants::VOLUME_IDENTIFIER_KIND_HAL_UDI;
pub use self::constants::VOLUME_IDENTIFIER_KIND_LABEL;
pub use self::constants::VOLUME_IDENTIFIER_KIND_NFS_MOUNT;
pub use self::constants::VOLUME_IDENTIFIER_KIND_UNIX_DEVICE;
pub use self::constants::VOLUME_IDENTIFIER_KIND_UUID;
pub use self::constants::VOLUME_MONITOR_EXTENSION_POINT_NAME;

#[doc(hidden)]
pub mod traits {
    pub use super::ActionExt;
    pub use super::ActionGroupExt;
    pub use super::ActionMapExt;
    pub use super::AppInfoExt;
    pub use super::AppLaunchContextExt;
    pub use super::ApplicationCommandLineExt;
    pub use super::ApplicationExt;
    pub use super::BufferedInputStreamExt;
    pub use super::BufferedOutputStreamExt;
    pub use super::CancellableExt;
    pub use super::CharsetConverterExt;
    pub use super::ConverterExt;
    pub use super::ConverterInputStreamExt;
    pub use super::ConverterOutputStreamExt;
    pub use super::DBusInterfaceExt;
    pub use super::DBusInterfaceSkeletonExt;
    pub use super::DBusObjectExt;
    pub use super::DBusProxyExt;
    pub use super::DataInputStreamExt;
    pub use super::DataOutputStreamExt;
    #[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(all(not(windows), not(target_os = "macos")))))]
    pub use super::DesktopAppInfoExt;
    pub use super::DriveExt;
    pub use super::EmblemedIconExt;
    pub use super::FileEnumeratorExt;
    pub use super::FileExt;
    pub use super::FileIOStreamExt;
    pub use super::FileInputStreamExt;
    pub use super::FileMonitorExt;
    pub use super::FileOutputStreamExt;
    pub use super::FilenameCompleterExt;
    pub use super::FilterInputStreamExt;
    pub use super::FilterOutputStreamExt;
    pub use super::IOStreamExt;
    pub use super::IconExt;
    pub use super::InetAddressExt;
    pub use super::InetAddressMaskExt;
    pub use super::InetSocketAddressExt;
    pub use super::InputStreamExt;
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    pub use super::ListModelExt;
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    pub use super::ListStoreExt;
    pub use super::LoadableIconExt;
    pub use super::MemoryInputStreamExt;
    #[cfg(any(feature = "v2_64", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
    pub use super::MemoryMonitorExt;
    pub use super::MemoryOutputStreamExt;
    pub use super::MenuAttributeIterExt;
    pub use super::MenuLinkIterExt;
    pub use super::MenuModelExt;
    pub use super::MountExt;
    pub use super::MountOperationExt;
    pub use super::NetworkAddressExt;
    pub use super::NetworkMonitorExt;
    pub use super::NetworkServiceExt;
    pub use super::OutputStreamExt;
    pub use super::PermissionExt;
    pub use super::PollableInputStreamExt;
    pub use super::PollableOutputStreamExt;
    pub use super::ProxyAddressExt;
    pub use super::ProxyExt;
    pub use super::ProxyResolverExt;
    pub use super::RemoteActionGroupExt;
    pub use super::ResolverExt;
    pub use super::SeekableExt;
    pub use super::SettingsBackendExt;
    pub use super::SettingsExt;
    pub use super::SocketAddressEnumeratorExt;
    pub use super::SocketAddressExt;
    pub use super::SocketClientExt;
    pub use super::SocketConnectableExt;
    pub use super::SocketConnectionExt;
    pub use super::SocketExt;
    pub use super::SocketListenerExt;
    pub use super::SocketServiceExt;
    pub use super::TcpConnectionExt;
    pub use super::ThreadedSocketServiceExt;
    pub use super::TlsBackendExt;
    pub use super::TlsCertificateExt;
    pub use super::TlsClientConnectionExt;
    pub use super::TlsConnectionExt;
    pub use super::TlsDatabaseExt;
    pub use super::TlsFileDatabaseExt;
    pub use super::TlsInteractionExt;
    pub use super::TlsPasswordExt;
    pub use super::TlsServerConnectionExt;
    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub use super::UnixFDListExt;
    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub use super::UnixInputStreamExt;
    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub use super::UnixOutputStreamExt;
    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub use super::UnixSocketAddressExt;
    pub use super::VfsExt;
    pub use super::VolumeExt;
    pub use super::VolumeMonitorExt;
    pub use super::ZlibCompressorExt;
    pub use super::ZlibDecompressorExt;
}
