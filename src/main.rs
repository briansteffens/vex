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
fn execute(commands: &Vec<String>, dry_run: bool)
{
    for command in commands
    {
        println!("vex: {}", command);

        if dry_run
        {
            continue;
        }

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

fn print_usage()
{
    println!("Usage: vex [options] \"some command\"");
    println!("Options:");
    println!("    --dry       - Output commands but don't execute them");
    println!("    --start=\"<\" - Customize pattern start character");
    println!("    --stop=\">\"  - Customize pattern stop character");
    println!("    --sep=\"|\"   - Customize pattern separator");
}

fn main()
{
    // Default options
    let mut dry_run: bool = false;
    let mut pattern_start = '[';
    let mut pattern_stop = ']';
    let mut pattern_separator = ',';

    // Leftover arguments after parsing options
    let mut args: Vec<String> = Vec::new();

    for arg in env::args().skip(1)
    {
        // Treat arguments that don't start with "--" as the command to run
        if !arg.starts_with("--")
        {
            args.push(arg);
            continue;
        }

        // Get the name out of the "--name=value" pattern
        let mut it = arg.chars().skip(2);
        let name: String = it.by_ref().take_while(|&a| a != '=').collect();

        // Process flags (no "=value" part)
        if name == "dry"
        {
            dry_run = true;
            continue;
        }

        // Get the value from the argument
        let value: String = it.collect();

        if value.len() != 1
        {
            print_usage();
            return;
        }

        let value_char = value.chars().next().unwrap();

        // Assign argument values to their proper variables
        match name.as_ref()
        {
            "start" => pattern_start = value_char,
            "stop" => pattern_stop = value_char,
            "sep" => pattern_separator = value_char,
            _ =>
            {
                print_usage();
                return;
            },
        }
    }

    if args.len() != 1
    {
        print_usage();
        return;
    }

    // Raw command pattern to process
    let ref raw = args[0];

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

    if patterns.len() == 0
    {
        println!("No patterns detected. Exiting.");
        return;
    }

    let commands = render(&parts, &permute(&patterns, 0, &Vec::new()));
    execute(&commands, dry_run);
}
