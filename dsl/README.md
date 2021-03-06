# Bobbin DSL

Tools for working with Bobbin DSL, a s-expression based domain-specific language for describing MCUs and Peripherals.

- [bobbin-chip](./bobbin-chip/) is a library for reading Bobbin DSL and a code generator that produces Rust crates.
- [bobbin-dsl](./bobbin-dsl/) reads [CMSIS-SVD](https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html) format files and generates Bobbin DSL files.
- [bobbin-sexp](./bobbin-sexp/) is a [s-expression](https://en.wikipedia.org/wiki/S-expression) parsing library with support for Rust-like numbers.

## Peripheral Prototype

A peripheral definition the [SAMD21 DAC](../mcu-src/bobbin-sam/samd21/periph/dac.rx). Comments are prefixed with a semicolon.

```
(peripheral
    ; signature: 0a8c569a91066543 

    ; The signature comment is auto-generated by bobbin-svd and is a hash of the
    ; registers and fields (but not reset values) of the peripheral.
    
    (group-name DAC) ; Each peripheral definition should include a group name.

    ; This peripheral declaration does not include a base address, which means that
    ; it is to be used as a prototype.

    (address-block ; The size and usage of the peripheral address space.
        (offset 0x0)
        (size 0x10)
        (usage registers)
    )
    (description "Digital Analog Converter")
    (register
        (name CTRLA)
        (offset 0x0) ; Offset in bytes from the peripheral base address.
        (size 0x8)   ; Size in bits of the register, uses the peripheral size if not specified.
        (description "Control A")

        ; All fields currently require a name, bit-offset, bit-width, and description.
        ; Future version could allow shorthand: (field NAME BIT-OFFSET BIT-WIDTH "DESCRIPTION")

        ; Registers may have an access attribute of read-only, read-write, etc.
        ; Access is inherited from the peripheral if not specified

        (field
            (name SWRST)
            (bit-offset 0) ; The offset in bits from the LSB end of the register.
            (bit-width 1)  ; The width of the field in bits.

            ; Fields may have an access attribute of read-only, read-write, etc.
            ; Access is inherited from the register if not specified.

            (description "Software Reset")
        )
        (field
            (name ENABLE)
            (bit-offset 1)
            (bit-width 1)
            (description "Enable")
        )
        (field
            (name RUNSTDBY)
            (bit-offset 2)
            (bit-width 1)
            (description "Run in Standby")
        )
    )
    (register
        (name CTRLB)
        (offset 0x1)
        (size 0x8)
        (description "Control B")
        (field
            (name EOEN)
            (bit-offset 0)
            (bit-width 1)
            (description "External Output Enable")
        )
        (field
            (name IOEN)
            (bit-offset 1)
            (bit-width 1)
            (description "Internal Output Enable")
        )
        (field
            (name LEFTADJ)
            (bit-offset 2)
            (bit-width 1)
            (description "Left Adjusted Data")
        )
        (field
            (name VPD)
            (bit-offset 3)
            (bit-width 1)
            (description "Voltage Pump Disable")
        )
        (field
            (name BDWP)
            (bit-offset 4)
            (bit-width 1)
            (description "Bypass DATABUF Write Protection")
        )
        (field
            (name REFSEL)
            (bit-offset 6)
            (bit-width 2)
            (description "Reference Selection")

            ; Field records may include value lists. 
            
            ; Currently these are not used for codegen because in some cases enums 
            ; are preferred but in others individual constants work better.

            ; Medium term plan is to have a way to define enums and constant lists
            ; at the peripheral level so that they can be used across registers
            ; and fields.

            (value
                (value "0x0")
                (name "INT1V")
                (description "Internal 1.0V reference")
            )
            (value
                (value "0x1")
                (name "AVCC")
                (description "AVCC")
            )
            (value
                (value "0x2")
                (name "VREFP")
                (description "External reference")
            )
        )
    )
    (register
        (name DATA)
        (offset 0x8)
        (size 0x10)
        (description "Data")

        (field
            (name DATA)
            (bit-offset 0)
            (bit-width 16)
            (description "Data value to be converted")
        )
    )

    ...

    ; The following register is read-only, with a single read-only field.

    (register
        (name STATUS)
        (offset 0x7)
        (size 0x8)
        (access read-only)
        (description "Status")
        (field
            (name SYNCBUSY)
            (bit-offset 7)
            (bit-width 1)
            (access read-only)
            (description "Synchronization Busy Status")
        )
    )
)
```

### Register and Field Arrays

Peripherals often include register arrays. The following STM32 DMA peripheral fragment shows how this is handled.

```
    ...

    (register
        (name SNDTR)        
        (dim 8) ; the number of items in this array        
        (dim-increment 0x18) ; the number to add to the offset for each item in the array        
        (offset 0x14) ; the offset of the first item in the array

        ; The offset of item N is (offset + (N * dim-increment))
        ; where N is zero-based and less than (dim).

        (size 0x20) ; the size in bits of the register
        (access read-write)
        (reset-value 0x0)
        (description "stream x number of data register")
        (field
            (name NDT)
            (bit-offset 0)
            (bit-width 16)
            (description "Number of data items to transfer")
        )
    )
    ...
```

Field arrays are handled similarly except that bit offsets are used:

```
    (register
        (name MODER)
        (offset 0x0)
        (size 0x20)
        (access read-write)
        (description "GPIO port mode register")
        (field
            (name MODER)
            (dim 16) ; the number of items in the array
            (dim-increment 2) ; the number to add to the bit offset for each item in the array
            (bit-offset 0) ; the offset of the first item in the array
            (bit-width 2) ; the width of each item in bits

            ; The bit-offset of item N is (bit-offset + (N * dim-increment))
            ; where N is zero-based and less than (dim)

            (description "Port x configuration bits (y = 0..15)")
        )
    )
```

## Peripheral Groups

Peripheral groups contain peripherals that share the same register definitions but that are located at different addresses. Often, a peripheral group is defined in the top-level file but refer to
a prototype file in a different file.

Here is a peripheral-group file for the DAC peripheral used in the example above:

```
    (peripheral-group
        ; signature: 0a8c569a91066543
        (name DAC)
        (prototype "periph/dac.rx") ; This is the file that contains the prototype peripheral
        (peripheral

            ; Each peripheral must have a name, which can be the same or different than the
            ; peripheral group. Typically peripheral groups that will only ever have one instance
            ; will have a peripheral with the same name, while peripheral groups that might
            ; have more than one instance will have peripherals named *BASE*0, *BASE*1
            ; or *BASE*A, *BASE*B, etc.

            (name DAC)
            (address 0x42004800) ; The base address of the peripheral

            ; Peripherals may have a (clock) section that indicate the primary
            ; input clock and define clock gates.

            (clock
                (input (name GCLK_DAC)) ; The name of the primary input clock;

                ; Gate definitions should include the gate type (often EN, RESET, SLEEP, or STOP)
                ; as well as the peripheral, register, and field used to enable that gate.

                (gate (type EN) (periph PM) (register APBCMASK) (field DAC))
            )

            ; Peripherals may define one or more signals. Each signal should include
            ; a name and type. If a peripheral only has one type of signal, use the
            ; the peripheral group name, otherwise use the peripheral group name 
            ; followed by an underscore as the prefix.

            ; Signal names should be the same as used in pin (altfn) definitions, described
            ; in a later section.

            ; Multiple peripherals of the same type should share the same signal type
            ; but have unique signal names.

            (signal (name DAC) (type DAC))

            ; Peripherals may have one or more interrupts. Each interrupt should include
            ; a name, type, and value. If a peripheral only has one type of interrupt, use
            ; the peripheral group name as the interrupt type, otherwise use the 
            ; peripheral group name followed by an underscore as the prefix.

            ; Multiple peripherals may share the same interrupt number.

            (interrupt
                (name DAC)
                (type DAC)
                (value 25)
            )
        )
    )
```

## Peripheral groups with module imports

Bobbin-DSL allows importing peripheral definitions from external modules. The
(module) definition is used for this.

```
    (peripheral-group
        (name CRC)

        ; Import peripheral, registers, and fields from the stm32_common::crc module.

        (module (name "::stm32_common::crc::*"))
        (peripheral
            (name CRC)
            (address 0x40023000)
            (clock
                (gate (type EN) (periph RCC) (register AHBENR) (field CRCEN))
            )            
            (description "Cyclic redundancy check calculation unit")
        )
    )
```

## Peripherals with Pins and Signals

Most MCUs have a single peripheral that manages pins and that contains some kind of way to
connect internal peripherals to pins. This is handled through a pin definition.

Each pin definitions contains a pin name and index and a list of altfn definitions. Each
altfn definition includes an index to select the altfn as well as the signal that
the pin will be connected to when the altfn is connected.

The signal names should be the same as those defined in the peripheral (signal) definitions.


```
        (peripheral 
            (name PORTA)
            (address 0x41004400)
            (clock
                (gate (type EN) (periph PM) (register APBBMASK) (field PORT))
            )               
            (pin (name PA00) (index 0)
                (altfn 0 EXTINT_0)
                (altfn 3 SERCOM1_PAD_0)
                (altfn 4 TCC2_WO_0)
            )
            (pin (name PA01) (index 1)
                (altfn 0 EXTINT_1)
                (altfn 3 SERCOM1_PAD_1)
                (altfn 4 TCC2_WO_1)
            )
            (pin (name PA02) (index 2)
                (altfn 0 EXTINT_2)
                (altfn 1 AIN_0)
                (altfn 1 DAC)
                (altfn 1 Y_0)
                (altfn 1 VOUT)
            )
            (pin (name PA03) (index 3)
                (altfn 0 EXTINT_3)
                (altfn 1 ADC)
                (altfn 1 VREFADAC)
                (altfn 1 VREFA)
                (altfn 1 AIN_1)
                (altfn 1 Y_1)
            )
            ...
        )
```

## Peripherals with Channels

Some peripherals such as DMA, ADC and Timers are best described as having mutiple logical channels.
Use (channel) to define these channels, which may have their own interrupts and signals.

An example of a DMA peripheral with multiple channels:

```
    (peripheral-group
        (name DMA)
        (module (name "::stm32_common::dma_f3::*"))
        (peripheral
            (name DMA1)
            (address 0x40020000)
            (clock
                (gate (type EN) (periph RCC) (register AHBENR) (field DMA1EN))
            )
            (description "DMA controller 1")

            ; Each channel must have a unique name and index. The name should match
            ; what is used in the data sheet, and the index should be whatever
            ; is most convenient at the register indexing level.

            (channel
                (name DMA1_CH1)
                (index 0)

                ; Channels may have their own interrupts and signals. 

                (interrupt
                    (name DMA1_CH1)
                    (type DMA)
                    (value 11)
                    (description "DMA1 channel 1 interrupt")
                )
            )
            (channel
                (name DMA1_CH2)
                (index 1)
                (interrupt
                    (name DMA1_CH2)
                    (type DMA)
                    (value 12)
                    (description "DMA1 channel 2 interrupt")
                )
            )

            ...
            
            (channel
                (name DMA1_CH7)
                (index 6)    
                (interrupt
                    (name DMA1_CH7)
                    (type DMA)
                    (value 17)
                    (description "DMA1 channel 7 interrupt")
                )
            )
        )

        ...
    
    )
```

## MCU Clocks

Each MCU definition should include a (clocks) definition that describes
the types of clock inputs, sources, and signals available in the clock tree.

**Inputs** are external clocks that are fed into the MCU. They may or
may not be present, and are assumed to be zero if not available.

**Sources** are internal clock sources. Some sources are fixed frequency,
some are variable frequency, and some are connected to **Inputs**.

**Outputs** are clock signals that may depend on other clocks.

```
    (clocks

        ; These are external clock inputs. min and max are
        ; here for reference but not currently used for codegen.

        (input (name OSC) (min 4000000) (max 26000000))
        (input (name OSC32) (min 32768) (max 32768))
        

        ; These are clock sources. 

        (source (name HSI) (speed 8000000))
        (source (name HSE) (input (name OSC)))
        (source (name LSI) (speed 40000))
        (source (name LSE) (input (name OSC32)))

        ; These outputs are not necessarily used by any peripheral
        ; but are convenient to have for debugging and to base
        ; other clock calculations off of.

        (output (name PLLCLK))                
        (output (name SYSCLK) (max 216000000)) ; max is here for reference but not enforced

        ; These are the primary clock outputs listed on the clock tree in the
        ; MCU reference.

        (output (name HCLK))
        (output (name SYSTICK))
        (output (name FHCLK))
        (output (name PCLK1))
        (output (name PCLK2))
        (output (name TIM_PCLK1))
        (output (name TIM_PCLK2))

        ; Although these are not listed on the clock tree in the MCU reference, they are
        ; included here because they are configurable inputs to specific peripherals.
        
        ; For instance, this MCU allows each I2C instance to be independently configured
        ; to use a specific clock as input.

        ; Including them here allows us to calculate the  specific clock value being sent to 
        ; the peripheral instead of putting clock calculations in the peripheral driver.

        (output (name I2C1))
        (output (name I2C2))
        (output (name I2C3))

        (output (name I2S2))
        (output (name I2S3))

        (output (name USBCLK))

        (output (name USART1))
        (output (name USART2))
        (output (name USART3))
        (output (name UART4))
        (output (name UART5))

        (output (name TIM1))
        (output (name TIM2))
        (output (name TIM3))
        (output (name TIM4))
        (output (name TIM8))
        (output (name TIM15))
        (output (name TIM16))
        (output (name TIM17))
        (output (name TIM20))

        (output (name RTCCLK))
        (output (name ADC12))
        (output (name ADC34))
    )    

```

## Variants

The (variants) record allows listing variants of the current device. For now,
variants are only used for setting up linker files with the appropriate
memory maps for each variant, but future versions will allow
conditional definition of registers and fields based on variant.

Typically only variants with different memory layouts and peripheral counts
are considered; differences in packaging are ignored.

```
    (variants
        (variant (name STM32F303xB) (link "f303xb.ld"))
        (variant (name STM32F303xC) (link "f303xc.ld"))
        (variant (name STM32F303xD) (link "f303xd.ld"))
        (variant (name STM32F303xE) (link "f303xe.ld"))
    )
```

## Crate Imports

At the device level, modules from external crates can be imported into 
the current device.

```
    (crate
        (name "stm32-common")
        (module (name bobbin_bits))
        (module (name bobbin_mcu))
        (module (name bobbin_hal))
        (module (name nvic))
        (module (name scb))
        (module (name systick))
        (module (name mpu))
        (module (name fpu))
        (module (name dcb))
        (module (name itm))
    )
```

# Top-Level Device

Here are excerpts from a typical top-level device definition:

```
(device
    (name STM32F3x)
    (size 0x20) ; the default register size

    ; the number of interrupts, this must be defined to generate placeholder
    ; interrupt handlers.
    (interrupt-count 82) 
    (description "STM32F3x")

    ; A list of device variants

    (variants
        (variant (name STM32F303xB) (link "f303xb.ld"))
        (variant (name STM32F303xC) (link "f303xc.ld"))
        (variant (name STM32F303xD) (link "f303xd.ld"))
        (variant (name STM32F303xE) (link "f303xe.ld"))
    )

    ; A crate import, in this case from the vendor-common crate for this device.

    (crate
        (name "stm32-common")
        (module (name bobbin_bits))
        (module (name bobbin_mcu))
        (module (name bobbin_hal))
        (module (name nvic))
        (module (name scb))
        (module (name systick))
        (module (name mpu))
        (module (name fpu))
        (module (name dcb))
        (module (name itm))
    )

    ; Clock definitions for this device.

    (clocks
        (input (name OSC) (min 4000000) (max 26000000))
        (input (name OSC32) (min 32768) (max 32768))
        
        (source (name HSI) (speed 8000000))
        (source (name HSE) (input (name OSC)))
        (source (name LSI) (speed 40000))
        (source (name LSE) (input (name OSC32)))
        (output (name PLLCLK))
        (output (name SYSCLK) (max 216000000))

        (output (name HCLK))
        (output (name SYSTICK))
        (output (name FHCLK))
        (output (name PCLK1))
        (output (name PCLK2))
        (output (name TIM_PCLK1))
        (output (name TIM_PCLK2))

        (output (name I2C1))
        (output (name I2C2))
        (output (name I2C3))

        (output (name I2S2))
        (output (name I2S3))

        (output (name USBCLK))

        (output (name USART1))
        (output (name USART2))
        (output (name USART3))
        (output (name UART4))
        (output (name UART5))

        (output (name TIM1))
        (output (name TIM2))
        (output (name TIM3))
        (output (name TIM4))
        (output (name TIM8))
        (output (name TIM15))
        (output (name TIM16))
        (output (name TIM17))
        (output (name TIM20))

        (output (name RTCCLK))
        (output (name ADC12))
        (output (name ADC34))
    )    

    ; Local peripherals - these are device-specific and only have one instance.

    (peripheral (include "periph/rcc.rx"))
    (peripheral (include "periph/syscfg.rx"))
    (peripheral (include "periph/flash.rx"))
    (peripheral (include "periph/pwr.rx"))

    ; External peripherals - imported from the vendor-common crate.

    ...

    (peripheral-group
        (name IWDG)
        (module (name "::stm32_common::iwdg::*"))
        (peripheral
            (name IWDG)
            (address 0x40003000)
            (clock
                (input (name LSI))
            )            
            (description "Independent watchdog")        
        )
    )

    (peripheral-group
        (name WWDG)
        (module (name "::stm32_common::wwdg::*"))
        (peripheral
            (name WWDG)
            (address 0x40002c00)
            (clock
                (input (name PCLK1))
                (gate (type RST) (periph RCC) (register APB1RSTR) (field WWDGRST))
                (gate (type EN) (periph RCC) (register APB1ENR) (field WWDGEN))
            )            
            (description "System window watchdog")
            (interrupt
                (name WWDG)
                (value 0)
                (type WWDG)
                (description "Window Watchdog interrupt")
            ) 
        )
    )    

    ...
)