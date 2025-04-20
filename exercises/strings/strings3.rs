// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // Hint: You can use the `trim` method on a string slice.
    // input.trim().to_string()
    let mut trim_left_idx = 0;
    let mut trim_right_idx = input.len();
    for c in input.chars() {
        if c == ' ' {
            trim_left_idx += 1;
        } else {
            break;
        }
    }
    for c in input.chars().rev() {
        if c == ' ' {
            trim_right_idx -= 1;
        } else {
            break;
        }
    }
    input[trim_left_idx..trim_right_idx].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // Hint: You can use the `replace` method on a string slice.
    // input.replace("cars", "balloons")
    let target = "cars";
    let replacement = "balloons";
    
    let mut result = String::new();
    let mut last_idx = 0;
    
    // 查找所有 "cars" 的实例并替换
    let chars: Vec<char> = input.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    
    for mut i in 0..=chars.len() - target_chars.len() {
        let mut is_match = true;
        
        // 检查当前位置是否匹配 "cars"
        for j in 0..target_chars.len() {
            if chars[i + j] != target_chars[j] {
                is_match = false;
                break;
            }
        }
        
        if is_match {
            // 添加前面的部分
            result.push_str(&input[last_idx..i]);
            // 添加替换文本
            result.push_str(replacement);
            // 更新上次匹配后的索引
            last_idx = i + target_chars.len();
            // 不需要继续检查已替换部分
            i += target_chars.len() - 1;
        }
    }
    
    // 添加最后一部分
    result.push_str(&input[last_idx..]);
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
