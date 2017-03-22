# proj_manager 💖

This is a simple project manager, which gives you data about your project, to help you know  your project better.

It's written in Rust.

I write this project to practise Rust programming.

For I'm a beginner to Rust, there must be some naive code, please don't hesitate to point it out :joy:.

## Get binary!

### Windows

Go to the release page, download the latest exe.

### Other

Clone this project and build it yourself. This project hasn't got any dependencies, so no worry.

## How to use

Create a config file in your local project root, named "proj_config".

Open the binary in your project root(where your proj_config file is), you'll see:

```
proj_manager v0.1.0, open source under GNU General Public License v3.0.
See: https://github.com/ice1000/proj_manager
Load success.
```

You can type 'help' to get help(the content is different in versions):

```
Commands:
data           -- print the meta data stored in the cofiguration file.
ls             -- print all the files.
exit           -- exit project manager.
help           -- print this doc.
line           -- see how many lines of code is here in your project.
```

You can see how many lines of code is here in your project:

```
In .\.gitignore              => 11 lines, 6 per line.
In .\install.bat             => 27 lines, 9 per line.
In .\install.sh              => 26 lines, 11 per line.
In .\LICENSE                 => 675 lines, 52 per line.
In .\proj_config             => 5 lines, 14 per line.
In .\README.md               => 92 lines, 19 per line.
In .\src\doc.go              => 10 lines, 11 per line.
In .\src\dp\core\check.go    => 262 lines, 17 per line.
In .\src\dp\core\check_test.go=> 37 lines, 21 per line.
In .\src\dp\core\equation.go => 46 lines, 16 per line.
In .\src\dp\core\gen.go      => 115 lines, 24 per line.
In .\src\dp\core\gen_test.go => 27 lines, 17 per line.
In .\src\dp\core\parse.go    => 137 lines, 20 per line.
In .\src\dp\core\parse_test.go=> 34 lines, 25 per line.
In .\src\dp\core\repl.go     => 86 lines, 19 per line.
In .\src\dp\core\style.go    => 16 lines, 15 per line.
In .\src\dp\msg\error.go     => 43 lines, 14 per line.
In .\src\dp\msg\error_test.go=> 21 lines, 19 per line.
In .\src\dp\util\algo\stack.go=> 52 lines, 13 per line.
In .\src\dp\util\algo\stack_test.go=> 23 lines, 11 per line.
In .\src\dp\util\sb\sb.go    => 83 lines, 19 per line.
In .\src\dp\util\sb\sb_test.go=> 32 lines, 17 per line.
In .\src\dp\util\util.go     => 62 lines, 16 per line.
In .\src\dp\util\util_test.go=> 170 lines, 14 per line.
In .\src\main.go             => 19 lines, 10 per line.
Total: 2111 lines of code.
```

You can customize the it's behavior by editing the cofiguration file:

```
ign:Cargo.lock
ign:.idea
ign:.git
ign-sfx:exe
name:your project name
```

key|value
:---|---:
ign|ignore files with given name
ign-sfx|ignore files with given suffix
name|you project name
