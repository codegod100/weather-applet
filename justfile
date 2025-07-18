default: build

build:
	cargo build --release

export NAME := 'one.weird.nandi.weather-applet'
export APPID := NAME

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / 'release' / NAME

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / NAME
desktop-dst := share-dst / 'applications' / APPID + '.desktop'
icon-dst := share-dst / 'icons/hicolor/scalable/apps' / APPID + '-symbolic.svg'
applet-sun-icon-dst := share-dst / 'icons/hicolor/scalable/apps' /APPID + '-sun-symbolic.svg'
applet-moon-icon-dst := share-dst / 'icons/hicolor/scalable/apps' /APPID + '-moon-symbolic.svg'

install:
	install -Dm0755 {{ bin-src }} {{ bin-dst }}
	install -Dm0644 data/one.weird.nandi.weather-applet.svg {{ icon-dst }}
	install -Dm0644 data/one.weird.nandi.weather-applet.desktop {{ desktop-dst }}
	install -Dm0644 data/one.weird.nandi.weather-applet-sun.svg {{ applet-sun-icon-dst }}
	install -Dm0644 data/one.weird.nandi.weather-applet-moon.svg {{ applet-moon-icon-dst }}

uninstall:
	rm {{ bin-dst }}
	rm {{ icon-dst }}
	rm {{ desktop-dst }}
	rm {{ applet-sun-icon-dst }}
	rm {{ applet-moon-icon-dst }}
