@tool

extends EditorScript

const Mvt = preload("res://addons/geo-tile-loader/vector_tile_loader.gd")

func _run():
	decode_tile()
	load_tiles()
	print("Tests finished")

func decode_tile():
	var tile = MvtTile.new()
	if tile.layers().size() != 0:
		printerr("tile.layer_count() != 0")

	tile = MvtTile.load("test/data/tile.mvt")
	var layers = tile.layers()
	if layers.size() != 1:
		printerr("tile.layers().size() != 1")

	var layer = layers[0]
	if layer.name() != "cities":
		printerr("layer.name() != cities")

	var features = layer.features()
	if features.size() != 68:
		printerr("features.size() != 68")

	var feature = features[1]

	if feature.id() != 0:
		printerr("feature.id() != 0")

	var type = feature.geom_type()
	if type["GeomType"] != "POINT":
		printerr("feature.geom_type() != POINT")

func load_tiles():
	var tile = Mvt.load_tile("test/data/tile.mvt")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	tile = Mvt.load_tile_gz("test/data/tile.mvt.gz")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

func large_tile():
	var tile = MvtTile.load("test/data/22949.mvt")
	var layers = tile.layers()
	if layers.size() != 7:
		printerr("tile.layers().size() != 7")

	var layer = layers[0]
	if layer.name() != "point":
		printerr("layer.name() != point")
