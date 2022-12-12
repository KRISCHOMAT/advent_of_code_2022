const fs = require("fs");
class File {
  constructor(input) {
    this.size = input[0];
    this.name = input[1];
  }
}

class Directory {
  constructor(name, parent) {
    this.name = name || "root";
    this.parent = parent || false;
    this.size = 0;
    this.files = [];
    this.children = [];
  }

  calcSize() {
    if (this.parent) {
      this.parent.size = 0;
      this.parent.files.forEach((file) => {
        this.parent.size += parseInt(file.size);
      });
      this.parent.children.forEach((child) => {
        this.parent.size += child.size;
      });
      this.parent.calcSize();
    }
  }
}

const inputArray = fs.readFileSync("./input.txt", "utf8").split("\n");
let directories = [new Directory("root")];
let current = directories[0];
let cmd;
let dest;
let total = 0;

inputArray.forEach((line) => {
  if (line.startsWith("$ cd")) {
    cmd = "cd";
    dest = line.split(" ")[2];
  } else if (line.startsWith("$ ls")) {
    cmd = "ls";
  }

  if (cmd === "ls" && line.startsWith("dir")) {
    directories.push(new Directory(line.split(" ")[1], current));
    current.children.push(directories.at(-1));
  } else if (cmd === "ls" && !line.startsWith("dir") && !line.startsWith("$")) {
    current.files.push(new File(line.split(" ")));
    current.size += parseInt(line.split(" ")[0]);
    current.calcSize();
  } else if (cmd === "cd" && dest === "/") {
    current = directories[0];
  } else if (cmd === "cd" && dest === "..") {
    current = current.parent;
  } else if (cmd === "cd") {
    current = current.children.find((dir) => dir.name === dest);
  }
});

directories.forEach((dir) => {
  if (dir.size <= 100000) {
    //console.log(dir.size);
    total += dir.size;
  }
});

console.log(`part 1: ${total}`);
