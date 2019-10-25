avr-gcc -Wall -Wextra -Werror -O2 -DF_CPU=16000000UL -mmcu=atmega328p sine_pwm.cpp && \
avr-objcopy -O ihex -R .eeprom a.out sine_pwm.hex && \
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:sine_pwm.hex
