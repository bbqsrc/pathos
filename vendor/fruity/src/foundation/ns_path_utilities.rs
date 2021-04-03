#![allow(non_upper_case_globals)]
#![allow(missing_docs)]

use crate::objc::NSUInteger;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct NSSearchPathDirectory(NSUInteger);
pub const NSApplicationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(1);
// unsupported applications, demonstration versions (Demos)
pub const NSDemoApplicationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(2);
// developer applications (Developer/Applications).
#[deprecated(note = "there is no one single Developer directory.")]
pub const NSDeveloperApplicationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(3);
// system and network administration applications (Administration)
pub const NSAdminApplicationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(4);
// various documentation, support, and configuration files, resources (Library)
pub const NSLibraryDirectory: NSSearchPathDirectory = NSSearchPathDirectory(5);
// developer resources (Developer)
#[deprecated(note = "there is no one single Developer directory.")]
pub const NSDeveloperDirectory: NSSearchPathDirectory = NSSearchPathDirectory(6);
// user home directories (Users)
pub const NSUserDirectory: NSSearchPathDirectory = NSSearchPathDirectory(7);
// documentation (Documentation)
pub const NSDocumentationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(8);
// documents (Documents)
pub const NSDocumentDirectory: NSSearchPathDirectory = NSSearchPathDirectory(9);
// location of CoreServices directory (System/Library/CoreServices)
pub const NSCoreServiceDirectory: NSSearchPathDirectory = NSSearchPathDirectory(10);
// location of autosaved documents (Documents/Autosaved)
pub const NSAutosavedInformationDirectory: NSSearchPathDirectory = NSSearchPathDirectory(11);
// location of user's desktop
pub const NSDesktopDirectory: NSSearchPathDirectory = NSSearchPathDirectory(12);
// location of discardable cache files (Library/Caches)
pub const NSCachesDirectory: NSSearchPathDirectory = NSSearchPathDirectory(13);
// location of application support files (plug-ins, etc) (Library/Application Support)
pub const NSApplicationSupportDirectory: NSSearchPathDirectory = NSSearchPathDirectory(14);
// location of the user's "Downloads" directory
pub const NSDownloadsDirectory: NSSearchPathDirectory = NSSearchPathDirectory(15);
// input methods (Library/Input Methods)
pub const NSInputMethodsDirectory: NSSearchPathDirectory = NSSearchPathDirectory(16);
// location of user's Movies directory (~/Movies)
pub const NSMoviesDirectory: NSSearchPathDirectory = NSSearchPathDirectory(17);
// location of user's Music directory (~/Music)
pub const NSMusicDirectory: NSSearchPathDirectory = NSSearchPathDirectory(18);
// location of user's Pictures directory (~/Pictures)
pub const NSPicturesDirectory: NSSearchPathDirectory = NSSearchPathDirectory(19);
// location of system's PPDs directory (Library/Printers/PPDs)
pub const NSPrinterDescriptionDirectory: NSSearchPathDirectory = NSSearchPathDirectory(20);
// location of user's Public sharing directory (~/Public)
pub const NSSharedPublicDirectory: NSSearchPathDirectory = NSSearchPathDirectory(21);
// location of the PreferencePanes directory for use with System Preferences (Library/PreferencePanes)
pub const NSPreferencePanesDirectory: NSSearchPathDirectory = NSSearchPathDirectory(22);
// location of the user scripts folder for the calling application (~/Library/Application Scripts/code-signing-id)
pub const NSApplicationScriptsDirectory: NSSearchPathDirectory = NSSearchPathDirectory(23);
// For use with NSFileManager's URLForDirectory:inDomain:appropriateForURL:create:error:
pub const NSItemReplacementDirectory: NSSearchPathDirectory = NSSearchPathDirectory(99);
// all directories where applications can occur
pub const NSAllApplicationsDirectory: NSSearchPathDirectory = NSSearchPathDirectory(100);
// all directories where resources can occur
pub const NSAllLibrariesDirectory: NSSearchPathDirectory = NSSearchPathDirectory(101);
// location of Trash directory
pub const NSTrashDirectory: NSSearchPathDirectory = NSSearchPathDirectory(102);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct NSSearchPathDomainMask(NSUInteger);
pub const NSUserDomainMask: NSSearchPathDomainMask = NSSearchPathDomainMask(1);
pub const NSLocalDomainMask: NSSearchPathDomainMask = NSSearchPathDomainMask(2);
pub const NSNetworkDomainMask: NSSearchPathDomainMask = NSSearchPathDomainMask(4);
pub const NSSystemDomainMask: NSSearchPathDomainMask = NSSearchPathDomainMask(8);
pub const NSAllDomainsMask: NSSearchPathDomainMask = NSSearchPathDomainMask(0x0ffff);
