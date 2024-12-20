# advent

My 🎅 [Advent of Code](https://adventofcode.com) solutions. Includes a runner
and benchmarker with free Christmas trees 🎄.

<img width="600" alt="image" src="https://user-images.githubusercontent.com/17109887/206856121-6a5078e3-5ebf-4973-b530-3639b30a2efa.png">

**BASED ON: https://github.com/rossmacarthur/advent**

## Getting started

The following commands use a Cargo alias `cargo advent` defined in
`.cargo/config.toml` to run the solutions. This tool will automatically fetch
the input for the puzzle and cache it locally when the solution is run for the
first time. This requires the `ADVENT_SESSION` environment variable to be set.
You can find this under the cookie name "session" in your logged in Advent of
Code browser session. This tool follows the automation guidelines on the
[/r/adventofcode wiki](https://www.reddit.com/r/adventofcode/wiki/faqs/automation).

```sh
export ADVENT_SESSION="533..."
```

Solutions can be run using the `run` command. Just pass in the year and day. For
example, the following will run the solution for 2020 day 18.

```
cargo advent -y 2020 -d 18 run
```

Tests can be run using the `test` subcommand.

```
cargo advent -y 2020 -d 18 test
```

Benchmarks can be run using the  `bench` subcommand.

```
cargo advent -y 2020 -d 18 bench
```

Extra arguments can be passed to both Cargo and the binary. Arguments after the
first `--` argument will be passed to Cargo and arguments after the second `--`
will be passed to the actual binary. For example if we wanted the JSON output of
the benchmark we could run the following.

```
cargo advent -y 2020 -d 18 bench -- --features=json -- --output json
```

All of the above will be built using `--release`.

### New solutions

Use the following to add a [template](./crates/cli/src/template.rs) for a new
solution.

```
cargo advent -y 2022 -d 1 new
```

Open the browser for the given problem

```
cargo advent -y 2020 -d 7 open
```

## Using the runner/benchmarker

You can use the provided runner and benchmarker for your own solutions. To get
started simply add the crate to the Cargo manifest for your solution.

```toml
[dependencies]
advent = { git = "https://github.com/rossmacarthur/advent" }
```

Then use the following as your main function.

```rust
fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
```

**Where**

- `parse_input` is a function that returns any type `I` implementing `Clone`.
- Each part function takes `I` as an argument and returns something implementing
  `Display`.

Finally, `cli()` will instantiate a command line interface and run the program.
Ordinary runs will run each part once and output the answers. Passing `--bench`
to the program will perform a benchmark.

That's all! You're free to structure your program however else you want. See
[template.rs](./crates/cli/src/template.rs) for the template I use or any of the
solutions in this crate for an example.

Run and benchmark output:

<img width="382" alt="screenshot of run output"
       src="https://user-images.githubusercontent.com/17109887/206855331-d5f2ee15-0245-40c8-a673-be4f89a225c4.png">

<img width="380" alt="screenshot of bench output"
       src="https://user-images.githubusercontent.com/17109887/206855396-26d868b1-a9e9-414d-b655-9b979e091b4e.png">


### Features

There are also some optional features which pull in some other crates.

- **`festive`** enables some festive ascii art and changes the default output to
  `--output festive`
- **`json`** supports JSON output using `--output json`, useful for collecting
  benchmark information
- **`prelude`** re-exports my prelude crate that can be imported using
  ```rust
  use advent::prelude::*;
  ```

They can be enabled in your Cargo manifest like this:

```toml
[dependencies]
advent = { git = "https://github.com/rossmacarthur/advent", features = ["festive", "json"] }
```

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
