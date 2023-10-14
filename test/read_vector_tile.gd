@tool

extends EditorScript

const Mvt = preload("res://addons/geo-tile-loader/vector_tile_loader.gd")

func _run():
	var tile = VectorTile.new()
	if tile.layers().size() != 0:
		printerr("tile.layer_count() != 0")

	tile = VectorTile.load("test/data/tile.mvt")
	var layers = tile.layers()
	if layers.size() != 1:
		printerr("tile.layers().size() != 1")

	var layer = layers[0]
	if layer.name() != "cities":
		printerr("layer.name() != cities")

	#tile = VectorTile.load("test/data/22949.mvt")
	#layers = tile.layers()
	#if layers.size() != 7:
	#	printerr("tile.layers().size() != 7")

	#layer = layers[0]
	#if layer.name() != "point":
	#	printerr("layer.name() != cities")

	tile = Mvt.load_tile("test/data/tile.mvt")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	tile = Mvt.load_tile_gz("test/data/tile.mvt.gz")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	print("Tests finished")
