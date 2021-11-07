enum Token {
    Dot,
    Plus,
    Minus,
    Digit,
    Letter,
    Error,
}

#[derive(Debug)]
enum State {
    Major,
    Minor,
    Patch,
    Prerelease,
    Build,
    Error,
    End,
}

fn token(c: char) -> Token {
    match c {
        '.' => Token::Dot,
        '+' => Token::Plus,
        '-' => Token::Minus,
        _ => {
            if c.is_numeric() {
                Token::Digit
            } else if c.is_alphabetic() {
                Token::Letter
            } else {
                Token::Error
            }
        }
    }
}

fn handle_major(cs: &str, curr_counter: usize) -> usize {
    let mut counter = curr_counter;

    for c in cs[curr_counter..].chars() {
        let t = token(c);

        match t {
            Token::Digit => {
                counter = counter + 1;
            }
            _ => {
                break;
            }
        }
    }

    counter
}

fn transition_major(cs: &str, counter: usize, state: &State) -> State {
    if let State::Major = state {
        if let Token::Dot = token(cs.chars().nth(counter).unwrap()) {
            State::Minor
        } else {
            State::Error
        }
    } else {
        State::Error
    }
}

fn extract_major(cs: &str, begin: usize, end: usize, state: &State) -> Option<u32> {
    if let State::Error = state {
        None
    } else {
        cs[begin..end].parse::<u32>().ok()
    }
}

fn handle_minor(cs: &str, curr_counter: usize) -> usize {
    let mut counter = curr_counter;

    for c in cs[curr_counter..].chars() {
        let t = token(c);

        match t {
            Token::Digit => {
                counter = counter + 1;
            }
            _ => {
                break;
            }
        }
    }

    counter
}

fn transition_minor(cs: &str, counter: usize, state: &State) -> State {
    if let State::Minor = state {
        if let Token::Dot = token(cs.chars().nth(counter).unwrap()) {
            State::Patch
        } else {
            State::Error
        }
    } else {
        State::Error
    }
}

fn extract_minor(cs: &str, begin: usize, end: usize, state: &State) -> Option<u32> {
    if let State::Error = state {
        None
    } else {
        cs[begin..end].parse::<u32>().ok()
    }
}

fn handle_patch(cs: &str, curr_counter: usize) -> usize {
    let mut counter: usize = curr_counter;

    for c in cs[curr_counter..].chars() {
        let t = token(c);

        match t {
            Token::Digit => {
                counter = counter + 1;
            }
            _ => {
                break;
            }
        }
    }

    counter
}

fn transition_patch(cs: &str, counter: usize, state: &State) -> State {
    if let State::Patch = state {
        if counter == cs.len() {
            State::End
        } else if let Token::Plus = token(cs.chars().nth(counter).unwrap()) {
            State::Build
        } else if let Token::Minus = token(cs.chars().nth(counter).unwrap()) {
            State::Prerelease
        } else {
            State::Error
        }
    } else {
        State::Error
    }
}

fn extract_patch(cs: &str, begin: usize, end: usize, state: &State) -> Option<u32> {
    if let State::Error = state {
        None
    } else {
        cs[begin..end].parse::<u32>().ok()
    }
}

fn handle_prerelease(cs: &str, curr_counter: usize) -> usize {
    let mut counter = curr_counter;

    for c in cs[curr_counter..].chars() {
        let t = token(c);
        match t {
            Token::Digit => {
                counter = counter + 1;
            }
            Token::Letter => {
                counter = counter + 1;
            }
            _ => {
                break;
            }
        }
    }

    counter
}

fn transition_prerelease(cs: &str, counter: usize, state: &State) -> State {
    if let State::Prerelease = state {
        if counter == cs.len() {
            State::End
        } else if let Token::Plus = token(cs.chars().nth(counter).unwrap()) {
            State::Build
        } else {
            State::Error
        }
    } else {
        State::Error
    }
}

fn extract_prerelease(cs: &str, begin: usize, end: usize, state: &State) -> Option<String> {
    if let State::Error = state {
        None
    } else {
        Some(cs[begin..end].to_string())
    }
}

fn handle_build(cs: &str, curr_counter: usize) -> usize {
    let mut counter: usize = curr_counter;

    for c in cs[curr_counter..].chars() {
        let t = token(c);

        match t {
            Token::Digit => {
                counter = counter + 1;
            }
            _ => {
                break;
            }
        }
    }

    counter
}

fn transition_build(cs: &str, counter: usize, state: &State) -> State {
    if let State::Build = state {
        if counter == cs.len() {
            State::End
        } else {
            State::Error
        }
    } else {
        State::Error
    }
}

fn extract_build(cs: &str, begin: usize, end: usize, state: &State) -> Option<u32> {
    if let State::Error = state {
        None
    } else {
        cs[begin..end].parse::<u32>().ok()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Semver {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Option<String>,
    build: Option<u32>,
}

fn is_semver(sv: &str) -> Option<Semver> {
    let mut state = State::Major;
    let mut begin = 0;

    let mut major: Option<u32> = None;
    let mut minor: Option<u32> = None;
    let mut patch: Option<u32> = None;
    let mut prerelease: Option<String> = None;
    let mut build: Option<u32> = None;

    loop {
        match state {
            State::Major => {
                let end = handle_major(sv, begin);
                state = transition_major(sv, end, &state);
                major = extract_major(sv, begin, end, &state);
                begin = end + 1;
            }
            State::Minor => {
                let end = handle_minor(sv, begin);
                state = transition_minor(sv, end, &state);
                minor = extract_minor(sv, begin, end, &state);
                begin = end + 1;
            }
            State::Patch => {
                let end = handle_patch(sv, begin);
                state = transition_patch(sv, end, &state);
                patch = extract_patch(sv, begin, end, &state);
                begin = end + 1;
            }
            State::Prerelease => {
                let end = handle_prerelease(sv, begin);
                state = transition_prerelease(sv, end, &state);
                prerelease = extract_prerelease(sv, begin, end, &state);
                begin = end + 1;
            }
            State::Build => {
                let end = handle_build(sv, begin);
                state = transition_build(sv, end, &state);
                build = extract_build(sv, begin, end, &state);
                begin = end + 1;
            }
            State::End => {
                return Some(Semver {
                    major: major?,
                    minor: minor?,
                    patch: patch?,
                    prerelease: prerelease,
                    build: build,
                });
            }
            State::Error => {
                return None;
            }
        }
    }
}

fn main() {
    let semvers = [
        "12.34.56",
        "11.22.54",
        "tr34.23.54",
        "12.34.56-a5b",
        "32.87.12",
        "32.87.12+12",
	"32.87.12+a12",
        "32.87.12-1a+12",
        "12.34.65-12ae+54",
        "12.34.65-12ae+4r5",
        "!Asdf",
        "65.34.56+65",
    ];

    let mut correct_semvers = semvers
        .iter()
        .map(|s| is_semver(s))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();
    correct_semvers.sort_by(|a, b| b.cmp(a));
    println!("correct {:?}", correct_semvers);
}
