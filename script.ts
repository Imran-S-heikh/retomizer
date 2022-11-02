const rules = require("atomizer/src/rules");
const fs = require("fs");
const rust_rules: string[] = [];

type Keyable = {
  [key: string]: string;
};

const end = "__END__";
const start = "__START__";

function rlr(text: string) {
  return text.replace(start, "left").replace(end, "right");
}

function processArguments(arguments: Keyable[]) {
  const argsmap = {};
  const argsarr: string[] = [];

  arguments.map((args) => {
    Object.keys(args).forEach((key) => {
      argsmap[key] = args[key];
    });
  });

  Object.keys(argsmap).forEach((key) => {
    let value: string | number = argsmap[key];

    if (typeof value === "string" && value.includes('"')) {
      value = `r#"${value}"#`;
    } else {
      value = `"${value}"`;
    }
    argsarr.push(`("${rlr(key)}", ${rlr(value)})`);
  });

  return `Some(HashMap::from([
                ${argsarr.join(",")}
            ])),`;
}

function processStyles(styles: Keyable) {
  const styles_rust: string[] = [];

  Object.keys(styles).forEach((key) => {
    const placeholder = styles[key].replace(
      /\$[0-9]+/g,
      (match) => `\${${match.replace("$", "")}}`
    );
    const style = `("${rlr(key)}", "${placeholder}")`;
    styles_rust.push(style);
  });

  return `Style::mapped([${styles_rust.join(",")}])`;
}

rules.forEach((rule) => {
  const arguments = rule.arguments ? processArguments(rule.arguments) : "None";
  const rust_rule = `
    Rule {
            matcher: "${rule.matcher}",
            name: "${rule.name}",
            param_tovalue: ${Boolean(rule.allowParamToValue)},
            styles: ${processStyles(rule.styles)},
            arguments: ${arguments}
    },`;

  rust_rules.push(rust_rule);
});

fs.writeFile("rules.txt", rust_rules.join(""), (err) => {
  if (err) {
    console.log(err);
    return;
  }
  console.log("Rules Generated Successfully 🎉🎉");
});
