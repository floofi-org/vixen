# CPU specification structure

For reference purposes, memory range `0xFE00` to `0xFFFF` contains read-only information about the CPU some program is running on. This information may be used for UI context or to enable/disable specific features depending on the CPU supported feature set.

The size of the specification structure is not static as it depends on the size of the encoded information.

## Reference
| Size     | ID                              | Name                          | Description                                               |
|----------|---------------------------------|-------------------------------|-----------------------------------------------------------|
| 1 byte   | `name_length`                   | Name Length                   | Length of the CPU name in characters                      |
| Variable | `name`                          | Name                          | The name of the CPU as an ASCII string                    |
| 2 bytes  | `id`                            | ID                            | A unique model identifier                                 |
| 1 byte   | `microarchitecture_length`      | Microarchitecture Length      | Length of the microarchitecture name in characters        |
| Variable | `microarchitecture`             | Microarchitecture             | The name of the microarchitecture the CPU is based on     |
| 1 byte   | `microarchitecture_name_length` | Microarchitecture Name Length | Length of the microarchitecture codename in characters    |
| Variable | `microarchitecture_name`        | Microarchitecture Name        | The codename of the microarchitecture the CPU is based on |
| 1 byte   | `data_width`                    | Data Width                    | The number of bytes the CPU can process at once           |
| 1 byte   | `address_width`                 | Address Width                 | The number of bytes the CPU can address                   |
| 2 bytes  | `max_ram`                       | Maximum RAM                   | The highest amount of RAM the CPU can address             |
| 2 bytes  | `microcode`                     | Microcode Version             | A unique version number for the CPU microcode             |
