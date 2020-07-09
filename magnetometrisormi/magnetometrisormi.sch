EESchema Schematic File Version 4
LIBS:magnetometrisormi-cache
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L paikkaanturi:BM1422AGMV U1
U 1 1 5D820CD8
P 3900 1450
F 0 "U1" H 3875 1615 50  0000 C CNN
F 1 "BM1422AGMV" H 3875 1524 50  0000 C CNN
F 2 "paikkaanturi:MLGA010V020A" H 3900 1450 50  0001 C CNN
F 3 "" H 3900 1450 50  0001 C CNN
	1    3900 1450
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR01
U 1 1 5D8219B1
P 2900 1550
F 0 "#PWR01" H 2900 1400 50  0001 C CNN
F 1 "+3.3V" V 2915 1678 50  0000 L CNN
F 2 "" H 2900 1550 50  0001 C CNN
F 3 "" H 2900 1550 50  0001 C CNN
	1    2900 1550
	0    -1   -1   0   
$EndComp
$Comp
L power:+3.3V #PWR02
U 1 1 5D8227C8
P 5050 1950
F 0 "#PWR02" H 5050 1800 50  0001 C CNN
F 1 "+3.3V" V 5065 2078 50  0000 L CNN
F 2 "" H 5050 1950 50  0001 C CNN
F 3 "" H 5050 1950 50  0001 C CNN
	1    5050 1950
	0    1    -1   0   
$EndComp
Wire Wire Line
	3200 1550 3000 1550
$Comp
L power:GND #PWR04
U 1 1 5D8238D5
P 4650 1550
F 0 "#PWR04" H 4650 1300 50  0001 C CNN
F 1 "GND" V 4655 1422 50  0000 R CNN
F 2 "" H 4650 1550 50  0001 C CNN
F 3 "" H 4650 1550 50  0001 C CNN
	1    4650 1550
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4650 1550 4550 1550
$Comp
L Device:C_Small C1
U 1 1 5D824594
P 3000 1350
F 0 "C1" H 2800 1400 50  0000 L CNN
F 1 "1u" H 2800 1300 50  0000 L CNN
F 2 "Capacitor_SMD:C_0402_1005Metric" H 3000 1350 50  0001 C CNN
F 3 "~" H 3000 1350 50  0001 C CNN
	1    3000 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 1450 3000 1550
Connection ~ 3000 1550
Wire Wire Line
	3000 1550 2900 1550
$Comp
L power:GND #PWR03
U 1 1 5D827BF2
P 3000 1250
F 0 "#PWR03" H 3000 1000 50  0001 C CNN
F 1 "GND" H 3005 1077 50  0000 C CNN
F 2 "" H 3000 1250 50  0001 C CNN
F 3 "" H 3000 1250 50  0001 C CNN
	1    3000 1250
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C3
U 1 1 5D82807A
P 4950 2150
F 0 "C3" H 5100 2100 50  0000 L CNN
F 1 "0.01u" H 5100 2200 50  0000 L CNN
F 2 "Capacitor_SMD:C_0402_1005Metric" H 4950 2150 50  0001 C CNN
F 3 "~" H 4950 2150 50  0001 C CNN
	1    4950 2150
	1    0    0    1   
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5D829D07
P 4950 2250
F 0 "#PWR06" H 4950 2000 50  0001 C CNN
F 1 "GND" H 4955 2077 50  0000 C CNN
F 2 "" H 4950 2250 50  0001 C CNN
F 3 "" H 4950 2250 50  0001 C CNN
	1    4950 2250
	-1   0    0    -1  
$EndComp
Wire Wire Line
	4950 2050 4950 1950
Connection ~ 4950 1950
Wire Wire Line
	4950 1950 5050 1950
$Comp
L Device:C_Small C2
U 1 1 5D82CB31
P 2050 1650
F 0 "C2" V 2000 1500 50  0000 L CNN
F 1 "1u" V 2000 1700 50  0000 L CNN
F 2 "Capacitor_SMD:C_0402_1005Metric" H 2050 1650 50  0001 C CNN
F 3 "~" H 2050 1650 50  0001 C CNN
	1    2050 1650
	0    -1   1    0   
$EndComp
Wire Wire Line
	3200 1650 2150 1650
$Comp
L power:GND #PWR05
U 1 1 5D82D778
P 1950 1650
F 0 "#PWR05" H 1950 1400 50  0001 C CNN
F 1 "GND" V 1955 1522 50  0000 R CNN
F 2 "" H 1950 1650 50  0001 C CNN
F 3 "" H 1950 1650 50  0001 C CNN
	1    1950 1650
	0    1    1    0   
$EndComp
NoConn ~ 4550 1650
NoConn ~ 4550 1750
Text Label 4550 1850 0    50   ~ 0
MAG_DRDY
$Comp
L Device:R R2
U 1 1 5D836B6D
P 2850 2150
F 0 "R2" H 2920 2196 50  0000 L CNN
F 1 "4k7" H 2920 2105 50  0000 L CNN
F 2 "Resistor_SMD:R_0402_1005Metric" V 2780 2150 50  0001 C CNN
F 3 "~" H 2850 2150 50  0001 C CNN
	1    2850 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	3200 1850 2850 1850
Wire Wire Line
	3200 1750 2600 1750
Wire Wire Line
	2600 1750 2600 2000
Wire Wire Line
	4550 1950 4650 1950
Wire Wire Line
	4650 1950 4650 2400
Wire Wire Line
	4650 2400 2850 2400
Wire Wire Line
	2600 2400 2600 2300
Connection ~ 4650 1950
Wire Wire Line
	4650 1950 4950 1950
Wire Wire Line
	2850 2300 2850 2400
Connection ~ 2850 2400
Wire Wire Line
	2850 2400 2600 2400
Wire Wire Line
	2850 2000 2850 1850
Wire Wire Line
	2600 1750 2500 1750
Connection ~ 2600 1750
Wire Wire Line
	2850 1850 2500 1850
Connection ~ 2850 1850
Text Label 2500 1750 2    50   ~ 0
MAG_SDA
Text Label 2500 1850 2    50   ~ 0
MAG_SCL
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 5D84B896
P 800 850
F 0 "#FLG0101" H 800 925 50  0001 C CNN
F 1 "PWR_FLAG" H 800 1023 50  0000 C CNN
F 2 "" H 800 850 50  0001 C CNN
F 3 "~" H 800 850 50  0001 C CNN
	1    800  850 
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0101
U 1 1 5D84BC35
P 800 850
F 0 "#PWR0101" H 800 600 50  0001 C CNN
F 1 "GND" H 805 677 50  0000 C CNN
F 2 "" H 800 850 50  0001 C CNN
F 3 "" H 800 850 50  0001 C CNN
	1    800  850 
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x01_Female J1
U 1 1 5D855A9C
P 2400 3600
F 0 "J1" V 2246 3648 50  0000 L CNN
F 1 "hole_sda" V 2337 3648 50  0000 L CNN
F 2 "paikkaanturi:hole" H 2400 3600 50  0001 C CNN
F 3 "~" H 2400 3600 50  0001 C CNN
	1    2400 3600
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Female J2
U 1 1 5D856486
P 2900 3600
F 0 "J2" V 2746 3648 50  0000 L CNN
F 1 "hole_scl" V 2837 3648 50  0000 L CNN
F 2 "paikkaanturi:hole" H 2900 3600 50  0001 C CNN
F 3 "~" H 2900 3600 50  0001 C CNN
	1    2900 3600
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Female J3
U 1 1 5D856A6D
P 3350 3600
F 0 "J3" V 3196 3648 50  0000 L CNN
F 1 "hole_drdy" V 3287 3648 50  0000 L CNN
F 2 "paikkaanturi:hole" H 3350 3600 50  0001 C CNN
F 3 "~" H 3350 3600 50  0001 C CNN
	1    3350 3600
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Female J4
U 1 1 5D85778C
P 3850 3600
F 0 "J4" V 3696 3648 50  0000 L CNN
F 1 "hole_gnd" V 3787 3648 50  0000 L CNN
F 2 "paikkaanturi:hole" H 3850 3600 50  0001 C CNN
F 3 "~" H 3850 3600 50  0001 C CNN
	1    3850 3600
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Female J5
U 1 1 5D857CB7
P 4300 3600
F 0 "J5" V 4146 3648 50  0000 L CNN
F 1 "hole_3V3" V 4237 3648 50  0000 L CNN
F 2 "paikkaanturi:hole" H 4300 3600 50  0001 C CNN
F 3 "~" H 4300 3600 50  0001 C CNN
	1    4300 3600
	0    1    1    0   
$EndComp
$Comp
L power:+3.3V #PWR09
U 1 1 5D85811F
P 4300 3400
F 0 "#PWR09" H 4300 3250 50  0001 C CNN
F 1 "+3.3V" H 4315 3573 50  0000 C CNN
F 2 "" H 4300 3400 50  0001 C CNN
F 3 "" H 4300 3400 50  0001 C CNN
	1    4300 3400
	-1   0    0    -1  
$EndComp
$Comp
L power:GND #PWR08
U 1 1 5D85878F
P 3850 3400
F 0 "#PWR08" H 3850 3150 50  0001 C CNN
F 1 "GND" H 3855 3227 50  0000 C CNN
F 2 "" H 3850 3400 50  0001 C CNN
F 3 "" H 3850 3400 50  0001 C CNN
	1    3850 3400
	1    0    0    1   
$EndComp
Text Label 3350 3400 1    50   ~ 0
MAG_DRDY
Text Label 2400 3400 1    50   ~ 0
MAG_SDA
Text Label 2900 3400 1    50   ~ 0
MAG_SCL
$Comp
L Connector:Conn_01x01_Female J7
U 1 1 5D85ECB6
P 3750 2550
F 0 "J7" H 3778 2576 50  0000 L CNN
F 1 "DEBUG_GND" H 3778 2485 50  0000 L CNN
F 2 "paikkaanturi:hole" H 3750 2550 50  0001 C CNN
F 3 "~" H 3750 2550 50  0001 C CNN
	1    3750 2550
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x01_Female J8
U 1 1 5D85F32E
P 3750 2750
F 0 "J8" H 3778 2776 50  0000 L CNN
F 1 "DEBUG_3V3" H 3778 2685 50  0000 L CNN
F 2 "paikkaanturi:hole" H 3750 2750 50  0001 C CNN
F 3 "~" H 3750 2750 50  0001 C CNN
	1    3750 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	3200 1950 3200 2200
$Comp
L power:+3.3V #PWR010
U 1 1 5D86DDDC
P 3550 2750
F 0 "#PWR010" H 3550 2600 50  0001 C CNN
F 1 "+3.3V" V 3565 2878 50  0000 L CNN
F 2 "" H 3550 2750 50  0001 C CNN
F 3 "" H 3550 2750 50  0001 C CNN
	1    3550 2750
	0    -1   1    0   
$EndComp
$Comp
L power:GND #PWR07
U 1 1 5D86EA96
P 3550 2550
F 0 "#PWR07" H 3550 2300 50  0001 C CNN
F 1 "GND" V 3555 2422 50  0000 R CNN
F 2 "" H 3550 2550 50  0001 C CNN
F 3 "" H 3550 2550 50  0001 C CNN
	1    3550 2550
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Female J6
U 1 1 5D86BD00
P 3400 2200
F 0 "J6" H 3428 2226 50  0000 L CNN
F 1 "ADDR_SELECT" H 3428 2135 50  0000 L CNN
F 2 "paikkaanturi:hole" H 3400 2200 50  0001 C CNN
F 3 "~" H 3400 2200 50  0001 C CNN
	1    3400 2200
	1    0    0    -1  
$EndComp
$Comp
L Device:R R1
U 1 1 5D837057
P 2600 2150
F 0 "R1" H 2670 2196 50  0000 L CNN
F 1 "4k7" H 2670 2105 50  0000 L CNN
F 2 "Resistor_SMD:R_0402_1005Metric" V 2530 2150 50  0001 C CNN
F 3 "~" H 2600 2150 50  0001 C CNN
	1    2600 2150
	1    0    0    -1  
$EndComp
$EndSCHEMATC