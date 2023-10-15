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

	var keys = layer.keys()
	if keys.size() != 1:
		printerr("keys.size() != 1")

	var values = layer.values()
	if values.size() != 68:
		printerr("values.size() != 68")

	var feature = features[1]
	#print(feature)

	if feature.id() != 0:
		printerr("feature.id() != 0")

	var tags = feature.tags(layer)
	if tags["name"] != "San Francisco":
		printerr("tags[name] != San Francisco")
	if tags.has("xxx"):
		printerr("tags.has(xxx) != false")

	var type = feature.geom_type()
	if type["GeomType"] != "POINT":
		printerr("feature.geom_type() != POINT")

	var geometry = feature.geometry_raw()
	if geometry != [9, 1310, 3166]:
		printerr("geometry != [9, 1310, 3166]")

	geometry = feature.geometry()
	if geometry != [[1, 655, 1583]]:
		printerr("geometry !=  [[1, 655, 1583]]")

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
