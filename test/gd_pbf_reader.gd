@tool

extends EditorScript

const VectorTile = preload("res://vector_tile.gd")

func read_tile(bytes) -> VectorTile.Tile:
	var tile = VectorTile.Tile.new()
	var rc = tile.from_bytes(bytes)
	if rc == VectorTile.PB_ERR.NO_ERRORS:
		return tile
	else:
		printerr("Error reading tile. PB_ERR=%d" % rc)
		return

func load_tile(fn) -> VectorTile.Tile:
	var bytes = FileAccess.get_file_as_bytes(fn)
	return read_tile(bytes)

func _run():
	var tile = load_tile("res://test/data/tile.mvt")
	# print(tile)
	if tile.get_layers().size() != 1:
		printerr("tile.get_layers().size() != 1")

	var layer = tile.get_layers()[0]
	# print(layer)
	if layer.get_name() != "cities":
		printerr("layer.get_name() != cities")

	var features = layer.get_features()
	if features.size() != 68:
		printerr("features.size() != 68")

	var keys = layer.get_keys()
	if keys.size() != 1:
		printerr("keys.size() != 1")

	var values = layer.get_values()
	if values.size() != 68:
		printerr("values.size() != 68")

	var feature = features[1]
	#print(feature)

	if feature._id.value != null:
		var _id = feature.get_id()

	var tags = feature.get_tags()
	if tags != [0, 1]:
		printerr("tags != [0, 1]")
	var key = keys[tags[0]]
	var value = values[tags[1]]
	if key != "name":
		printerr("key != name")
	if value.get_string_value() != "San Francisco":
		printerr("value != San Francisco")
	if value._float_value.value != null:
		printerr("value._float_value != null")

	var type = feature.get_type()
	if type != VectorTile.Tile.GeomType.POINT:
		printerr("type != VectorTile.Tile.GeomType.POINT")

	var geometry = feature.get_geometry()
	if geometry != [9, 1310, 3166]:
		printerr("geometry != [9, 1310, 3166]")

	print("Tests finished")
