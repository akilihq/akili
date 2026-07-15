
use akili_core::{Tool, ToolSet};
use serde_json::{json, Value};
use std::{env, fs};

struct ReadSpan;

impl Tool for ReadSpan {
    fn name(&self) -> &str {
        "read_span"
    }

    fn description(&self) -> &str {
        "Return exact source lines of a file from from_line to to_line (inclusive)."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path":      { "type": "string"  },
                "from_line": { "type": "integer" },
                "to_line":   { "type": "integer" }
            },
            "required": ["path", "from_line", "to_line"]
        })
    }

    fn call(&self, args: Value) -> Result<Value, String> {
        let path = args["path"].as_str().ok_or("`path` must be a string")?;
        let from = args["from_line"].as_u64().ok_or("`from_line` must be an integer")? as usize;
        let to = args["to_line"].as_u64().ok_or("`to_line` must be an integer")? as usize;
        if from == 0 {
            return Err("lines are 1-based".into());
        }

        let text = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
        let lines: Vec<&str> = text.lines().collect();
        let end = to.min(lines.len());
        if from > end {
            return Err("from_line is past the end of the file".into());
        }

        let code = lines[from - 1..end]
            .iter()
            .enumerate()
            .map(|(i, l)| format!("{}\t{}", from + i, l))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(json!({ "file": path, "from": from, "to": end, "code": code }))
    }
}


struct CodingTools;

impl ToolSet for CodingTools {
    fn name(&self) -> &str {
        "coding"
    }
    fn tools(&self) -> Vec<Box<dyn Tool>> {
        // add RepoMap, Search, FindDefinition here as you build them
        vec![Box::new(ReadSpan)]
    }
}

fn registry() -> Vec<Box<dyn ToolSet>> {
    // Adding a whole new domain later is literally one line:
    //   sets.push(Box::new(EmailTools));
    vec![Box::new(CodingTools)]
}


fn all_tools() -> Vec<Box<dyn Tool>> {
    registry().into_iter().flat_map(|set| set.tools()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("list") => {
            for t in all_tools() {
                println!("{:18}{}", t.name(), t.description());
            }
        }
        Some("call") => {
            let name = match args.get(2) {
                Some(n) => n,
                None => return eprintln!("usage: akili call <tool> '<json-args>'"),
            };
            let raw = args.get(3).map(String::as_str).unwrap_or("{}");
            let parsed: Value = match serde_json::from_str(raw) {
                Ok(v) => v,
                Err(e) => return eprintln!("args must be valid JSON: {e}"),
            };
            let tools = all_tools();
            let tool = match tools.iter().find(|t| t.name() == name) {
                Some(t) => t,
                None => return eprintln!("no such tool: {name}"),
            };
            match tool.call(parsed) {
                Ok(out) => println!("{}", serde_json::to_string_pretty(&out).unwrap()),
                Err(e) => eprintln!("error: {e}"),
            }
        }
        _ => eprintln!("usage:\n  akili list\n  akili call <tool> '<json-args>'"),
    }
}
