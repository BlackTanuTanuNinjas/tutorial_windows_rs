// Windowsでコンソールを開かずに実行
#![cfg_attr(windows, windows_subsystem = "windows")]

use windows::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxW(None, w!("末尾WはWin32APIのunicode対応!"), w!("タイトル"), MB_OK);
        
        ShellMessageBoxW(None, None, 
            w!("ShellMessageBoxではアイコンと効果音が設定可能!\n4つ目の引数で、アイコンと効果音が変わります"),
            w!("Win32 APIはw!でテキストを設定"), 
            MB_ICONERROR);

        ShellMessageBoxW(None, None, w!("MB_* で他のアイコンも設定可能"), w!("World"), MB_ICONWARNING);
    }
}