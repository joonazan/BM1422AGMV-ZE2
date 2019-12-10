module wall ()
{
    cylinder(23, 9, 9);
}

module hole ()
{
    translate([0, 0, 3]) {
        cylinder(20, 6, 6);
    }
}

difference() {
    union() {
        hull() {
            wall();
            translate([50, 0, 0]) {
                wall();
            }
        }
        
        hull() {
            wall();
            translate([0, 50, 0]) {
                wall();
            }
        }
        
        hull() {
            translate([0, 50, 0]) {
                wall();
            }
            translate([50, 50, 0]) {
                wall();
            }
        }

        hull() {
            translate([50, 50, 0]) {
                wall();
            }
            translate([50, 50, 30]) {
                wall();
            }
        }
    }
    
    hole();
    translate([50, 0, 0]) {
        hole();
    }
    translate([0, 50, 0]) {
        hole();
    }
    translate([50, 50, 30]) {
        hole();
    }
}