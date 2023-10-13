@tool

extends EditorScript

func load_tile(fn) -> VectorTile:
	var bytes = FileAccess.get_file_as_bytes(fn)
	return VectorTile.read(bytes)

func load_tile_gz(fn) -> VectorTile:
	var bytes_gz = FileAccess.get_file_as_bytes(fn)
	var bytes = bytes_gz.decompress_dynamic(-1, FileAccess.COMPRESSION_GZIP)
	return VectorTile.read(bytes)

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

	tile = load_tile("test/data/tile.mvt")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	tile = load_tile_gz("test/data/tile.mvt.gz")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	print("Tests finished")