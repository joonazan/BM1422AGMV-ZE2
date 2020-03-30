with import <nixpkgs> {};

let
bluepy = python37.pkgs.buildPythonPackage rec {
  pname = "bluepy";
  version = "1.3.0";

  src = python37.pkgs.fetchPypi {
    inherit pname version;
    sha256 = "1v0wjy1rz0rbwghr1z3xhdm06lqn9iig6vr5j2wmymh3w6pysw9a";
  };

  buildInputs = [ glib.dev pkgconfig ];
  nativeBuildInputs = [ glib.dev pkgconfig ];
  #propagatedBuildInputs = [ pkgs.pkgconfig ];

  # run tests?
  doCheck = false;

  meta = with stdenv.lib; {
    description = "Python module for interfacing with BLE devices through Bluez";
    homepage = https://github.com/IanHarvey/bluepy;
    #license = licenses.gplv2;
  };
};
in stdenv.mkDerivation {
  name = "position-sensor-dev-env";
  buildInputs = [
    # for receiving BLE
    (python37.withPackages (ps: [bluepy]))

    # for building the visualization
    cmake pkg-config freetype expat
  ];
}
