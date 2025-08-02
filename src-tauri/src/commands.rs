use std::ffi::c_void;
use tokio::time::{sleep, Duration};
use windows::Win32::{
    Foundation::HGLOBAL,
    System::{
        DataExchange::CloseClipboard,
        Memory::{GlobalLock, GlobalUnlock},
    },
    UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
        KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_RETURN,
    },
};
use windows::Win32::{
    Foundation::HWND,
    System::DataExchange::{GetClipboardData, OpenClipboard},
};

// 定义要固定输出的文本内容
const FIXED_TEXT: &str = "坐席应做到：始终保持语气谦逊，态度诚恳，不与客户顶撞，发生争执。语调亲切、委婉，应面带微笑，使客户能感受到我们的关心和帮助。语速适中，对快语速或慢语速的客户，在一定适配区间内尝试接近他们的语速，缩小与客户的距离。发音清晰、准确，确保客户能听清我们的解答。客户通话中有投诉需求时，要以虚心态度仔细聆听，优先安抚情绪并及时致歉，切不可拒绝或中断通话。";

// 获取固定文本（转换为UTF-16编码）
fn get_fixed_text() -> Result<Vec<u16>, &'static str> {
    let wide_chars: Vec<u16> = FIXED_TEXT.encode_utf16().collect();
    Ok(wide_chars)
}

#[tauri::command]
pub async fn paste(stand: u32, float: u32) -> Result<(), &'static str> {
    // 使用固定文本替换剪贴板内容
    let utf16_units: Vec<u16> = get_fixed_text()?;
    
    for item in utf16_units {
        if item == 10 {
            // 处理换行符（回车）
            let input = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_RETURN,
                            wScan: 0,
                            dwFlags: KEYBD_EVENT_FLAGS(0),
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_RETURN,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];
            unsafe {
                SendInput(&input, std::mem::size_of::<INPUT>() as i32);
            }
        } else {
            // 处理普通字符
            let input = [INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: item,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            }];
            unsafe {
                SendInput(&input, std::mem::size_of::<INPUT>() as i32);
            }
        };

        // 随机延迟处理
        let random = rand::random::<u32>();
        sleep(Duration::from_millis((stand + random % float) as u64)).await;
    }

    Ok(())
}
