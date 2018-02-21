# git_track

Report time spent on given git project

~~~bash
$ git_track
    master             0.8
    develop            3.5
    feature/fatec_dae  21.5
~~~

You can remove log for a specifi branch using `--delete`

~~~bash
$ git_track --delete master
    develop            3.5
    feature/fatec_dae  21.5
~~~

## Installation

~~~bash
$ cargo install git_track
~~~


