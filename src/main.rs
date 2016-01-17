use std::process::{Command, Stdio};
use std::env;

// Recursively calculate all unique combinations of patterns
fn permute(patterns: &Vec<Vec<String>>, 
           index: usize, 
           base: &Vec<String>)
           -> Vec<Vec<String>>
{
    let mut ret: Vec<Vec<String>> = Vec::new();

    for option in &patterns[index]
    {
        let mut new_base = base.clone();

        new_base.push(option.clone());

        if index < patterns.len() - 1
        {
            // Continue recursion
            for res in permute(&patterns, index + 1, &new_base)
            {
                ret.push(res);
            }
        }
        else
        {
            // Deepest level of recursion
            ret.push(new_base);
        }
    }

    return ret;
}

// Combine a list of permutations with static parts to render all commands
fn render(parts: &Vec<String>, permutations: &Vec<Vec<String>>) -> Vec<String>
{
    let mut ret: Vec<String> = Vec::new();

    for permutation in permutations
    {
        let mut out = String::new();

        for i in 0..parts.len()
        {
            out = out + &parts[i].clone();

            if permutation.len() >= i + 1
            {
                out = out + &permutation[i].clone();
            }
        }

        ret.push(out);
    }

    return ret;
}

// Execute a list of commands
fn execute(commands: &Vec<String>)
{
    for command in commands
    {
        println!("vex: {}", command);

        let success =
            Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();

        if !success
        {
            break;
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1
    {
        println!("Usage: vex \"some command\"");
        return;
    }

    // Raw command pattern to process
    let ref raw = args[0];

    let pattern_start = '[';
    let pattern_stop = ']';
    let pattern_separator = ',';

    // Static parts of the command pattern
    let mut parts: Vec<String> = Vec::new();
    let mut part = String::new();

    // Dynamic (replaceable) parts of the command pattern
    let mut patterns: Vec<Vec<String>> = Vec::new();
    let mut pattern: Vec<String> = Vec::new();
    let mut option = String::new();

    // Keep track of whether we're in a [pattern,or] not
    let mut in_pattern = false;

    for c in raw.chars()
    {
        if c == pattern_start
        {
            // Beginning of a pattern, ex: [
            in_pattern = true;

            parts.push(part);
            part = String::new();
        }
        else if c == pattern_stop
        {
            // End of a pattern, ex: ]
            in_pattern = false;

            pattern.push(option);
            option = String::new();

            patterns.push(pattern);
            pattern = Vec::new();
        }
        else if in_pattern
        {
            // Inside a pattern
            if c == pattern_separator
            {
                // Pattern separator (ex: ,) - next option
                pattern.push(option);
                option = String::new();
            }
            else
            {
                // Add to current option
                option.push(c);
            }
        }
        else
        {
            // Add to current static part
            part.push(c);
        }
    }

    if part.len() > 0
    {
        parts.push(part);
    }

    let base: Vec<String> = Vec::new();
    let permutations = permute(&patterns, 0, &base);

    let commands = render(&parts, &permutations);

    execute(&commands);
}
