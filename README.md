# git_track

![Crate sticker](https://img.shields.io/crates/v/git_track.svg)

Log time spent on each branch for a Git repository.

## Usage

Start to `watch` your repository. This will create  *.git_track.log* file. Each minute, **Git_track** will log your current branch in this file.

~~~bash
$ git_track --watch
~~~

Then, to get a report juste enter `git_track`:

~~~bash
$ git_track
    master             0.8
    develop            3.5
    feature/fatec_dae  21.5
~~~

You can remove log for a specifi branch using `--delete`:

~~~bash
$ git_track --delete master
    develop            3.5
    feature/fatec_dae  21.5
~~~

## Installation

~~~bash
$ cargo install git_track
~~~
