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

// Calc Part 1
let part1 = 0;
let total = 0;
directories.forEach((dir) => {
  total += dir.size;
  if (dir.size <= 100000) {
    part1 += dir.size;
  }
});

console.log(`part 1: ${part1}`);

// Calc Part 2
const totalDiskSpace = 70000000;
const neededSpace = 30000000;
const unusedSpace = totalDiskSpace - directories[0].size;
const minSize = neededSpace - unusedSpace;

let min = directories[0].size;
directories.forEach((dir) => {
  if (dir.size >= minSize && dir.size < min) {
    min = dir.size;
  }
});

console.log(`part 2: ${min}`);
