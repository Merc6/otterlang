# OtterLang Standard Library Documentation

## Overview

The OtterLang standard library provides essential functionality for common programming tasks.

## Modules

### `io`

Input/output operations.

#### Functions

##### `print(message: string) -> unit`

Prints a message to stdout.

```otter
print("Hello, World!")
```

##### `println(message: string) -> unit`

Prints a message to stdout followed by a newline.

```otter
println("Hello, World!")
```

##### `println() -> unit`

Prints a newline to stdout.

```otter
println()  # Just prints a newline
```

##### `eprintln(message: string) -> unit`

Prints a message to stderr followed by a newline.

```otter
eprintln("Something went wrong")
```

##### `read_line() -> string | nil`

Reads a line from stdin. Returns `nil` on EOF.

```otter
line = read_line()
if line != nil:
    println(f"You entered: {line}")
```

### `math`

Mathematical functions.

#### Functions

##### `sqrt(x: float) -> float`

Returns the square root of x.

##### `sin(x: float) -> float`

Returns the sine of x (in radians).

##### `cos(x: float) -> float`

Returns the cosine of x (in radians).

##### `pow(base: float, exponent: float) -> float`

Returns base raised to the power of exponent.

##### `abs(x: float) -> float`

Returns the absolute value of x.

### `time`

Time and date operations.

#### Functions

##### `now_ms() -> int`

Returns the current time in milliseconds since epoch.

```otter
timestamp = now_ms()
```

##### `sleep_ms(ms: int) -> unit`

Sleeps for the specified number of milliseconds.

```otter
sleep_ms(1000)  # Sleep for 1 second
```

### `json`

JSON parsing and generation.

#### Functions

##### `parse(json_str: string) -> dict | array | nil`

Parses a JSON string into a dictionary or array.

```otter
data = json.parse('{"name": "Otter", "age": 42}')
```

##### `stringify(value: dict | array) -> string`

Converts a dictionary or array to a JSON string.

```otter
json_str = json.stringify({"key": "value"})
```

**Note:** For general value-to-string conversion, use the built-in `str()` function (`stringify()` remains available as a deprecated alias):

```otter
num_str = str(42)  # "42"
```

### `runtime`

Runtime utilities.

#### Functions

##### `collect_garbage() -> int`

Manually triggers garbage collection. Returns bytes freed.

```otter
freed = runtime.collect_garbage()
```

##### `gc.alloc(size: int) -> i64`

Allocates `size` bytes on the GC‑managed heap and returns a pointer (as an integer).

**Parameters:**
- `size`: Number of bytes to allocate.

**Example:**
```otter
let ptr = gc.alloc(128)
```

##### `gc.add_root(ptr: i64) -> unit`

Registers the given pointer as a GC root.

**Parameters:**
- `ptr`: Pointer returned by `gc.alloc`.

**Example:**
```otter
gc.add_root(ptr)
```

##### `gc.remove_root(ptr: i64) -> unit`

Removes a previously registered root pointer.

**Parameters:**
- `ptr`: Pointer previously added with `gc.add_root`.

**Example:**
```otter
gc.remove_root(ptr)
```

Manually triggers garbage collection. Returns bytes freed.

```otter
freed = runtime.collect_garbage()
```

##### `memory_profiler_start() -> unit`

Starts memory profiling.

```otter
runtime.memory_profiler_start()
```

##### `memory_profiler_stop() -> unit`

Stops memory profiling.

```otter
runtime.memory_profiler_stop()
```

##### `memory_profiler_stats() -> string`

Returns memory profiling statistics as JSON.

```otter
stats = runtime.memory_profiler_stats()
```

### `task`

Concurrent task execution.

#### Functions

##### `spawn(block: () -> T) -> Task<T>`

Spawns a concurrent task.

```otter
task = spawn:
    # concurrent code
    return result

value = await task
```

##### `await(task: Task<T>) -> T`

Waits for a task to complete and returns its result.

See [Tutorial Series](./TUTORIALS.md) for more examples.
