# git_track

Report time spent on given git project

~~~bash
$ ./git_track
    master             0.8
    develop            3.5
    feature/fatec_dae  21.5
~~~

You can remove log for a specifi branch using `--delete`

~~~bash
$ ./git_track --delete master
    develop            3.5
    feature/fatec_dae  21.5
~~~

## Installation

Install this Crontab _(replace `<your_project_path>`)_. To do, run `crontab -e` & copy this line:

~~~bash
* * * * * cd <your_project_path> && git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \\(.*\\)/\\1/' >> <your_project_path>/.tickets_count.log
~~~

Then clone this repository & build project with Cargo

~~~bash
$ git clone https://github.com/madeindjs/git_track.git
$ cd git_track
$ cargo build --release
~~~

Then move binary built in `<your_project_path>`

~~~bash
$ cp target/release/git_track <your_project_path>
~~~



