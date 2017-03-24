# proj_manager 💖

This is a simple project manager, which gives you data about your project, to help you know your project better.

It's written in Rust.

I write this project to practise Rust programming.

For I'm a beginner to Rust, there must be lots of naive code, please don't hesitate to point it out!

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
Load success DA★ZE.
```

You can type 'help' to get help(the content is different in versions):

```
Commands:
data           -- print the meta data stored in the cofiguration file.
ls             -- print all the files.
exit           -- exit project manager.
help           -- print this doc.
line           -- see how many lines of code is here in your project.
git            -- print git status.
```

You can see how many lines of code is here in your project:

```
In .\.gitignore                     => 8    lines, 5    per line.
In .\build.bat                      => 14   lines, 35   per line.
In .\Cargo.toml                     => 8    lines, 20   per line.
In .\clean.bat                      => 10   lines, 41   per line.
In .\LICENSE                        => 675  lines, 52   per line.
In .\proj_config                    => 13   lines, 12   per line.
In .\src\bin\command.rs             => 12   lines, 18   per line.
In .\src\bin\string_apis.rs         => 8    lines, 11   per line.
In .\src\config.rs                  => 70   lines, 24   per line.
In .\src\files.rs                   => 34   lines, 24   per line.
In .\src\funcs.rs                   => 135  lines, 23   per line.
In .\src\main.rs                    => 52   lines, 22   per line.
In .\src\model.rs                   => 84   lines, 18   per line.
Total: 1123 lines of code.
```

You can even customize the indent, just add config 'idt-line-1', 'idt-line-2', 'idt-line-3' in your config file.

You can customize the it's behavior by editing the cofiguration file:

```
ign:Cargo.lock
ign:.idea
ign:.git
ign-sfx:exe
name:your project name
idt-line-1:25
idt-line-2:3
idt-line-3:2
idt-ls-1:16
```

key|value
:---|---:
ign|ignore files with given name
ign-sfx|ignore files with given suffix
name|you project name