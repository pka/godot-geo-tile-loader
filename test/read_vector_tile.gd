@tool

extends EditorScript

const Mvt = preload("res://addons/geo-tile-loader/vector_tile_loader.gd")

func _run():
	decode_tile()
	load_tiles()
	large_tile()
	print("Tests finished")

func decode_tile():
	var tile = Mvt.load_tile("test/data/tile.mvt")
	var layers = tile.layers()
	if layers.size() != 1:
		printerr("tile.layers().size() != 1")

	var layer = layers[0]
	if layer.name() != "cities":
		printerr("layer.name() != cities")

	if layer.extent() != 4096:
		printerr("layer.extent() != 4096", layer.extent())

	var features = layer.features()
	if features.size() != 68:
		printerr("features.size() != 68")

	var feature = features[1]

	if feature.id() != 0:
		printerr("feature.id() != 0")

	var tags = feature.tags(layer)
	#print(tags)
	if tags["name"] != "San Francisco":
		printerr("tags[name] != San Francisco")
	if tags.has("xxx"):
		printerr("tags.has(xxx) != false")

	var type = feature.geom_type()
	if type["GeomType"] != "POINT":
		printerr("feature.geom_type() != POINT")

	#var geometry = feature.geometry_raw()
	#if geometry != [9, 1310, 3166]:
	#	printerr("geometry != [9, 1310, 3166]")

	var geometry = feature.geometry()
	if geometry != [[1, 655, 1583]]:
		printerr("geometry !=  [[1, 655, 1583]]")

	# Linestring example (https://github.com/mapbox/vector-tile-spec/tree/master/2.1#4353-example-linestring):
	# [[1, 2, 2], [2, 0, 8, 8, 0]]

func load_tiles():
	var tile = Mvt.load_tile_gz("test/data/tile.mvt.gz")
	if tile.layers().size() != 1:
		printerr("tile.layers().size() != 1")

	# Native load from file
	tile = MvtTile.load("test/data/tile.mvt")

func large_tile():
	# Tile 16/34323/22949
	var tile = MvtTile.load("test/data/22949.mvt")
	var layers = tile.layers()
	if layers.size() != 7:
		printerr("tile.layers().size() != 7")

	var layer = layers[0]
	if layer.name() != "point":
		printerr("layer.name() != point")

	if layer.extent() != 65536:
		printerr("layer.extent() != 65536")

	layer = layers[5]
	if layer.name() != "buildings":
		printerr("layer.name() != buildings")

	for feature in layer.features():
		var tags = feature.tags(layer)
		if tags.has("name") and tags["name"] == "Pfrundhaus":
			# print(tags)
			var reftags = { "type": "building", "osmId": 24910306, "levels": 3, "buildingType": "residential", "roofType": "hipped", "roofLevels": 1 }
			for key in reftags.keys():
				if tags[key] != reftags[key]:
					printerr("tags[", key , "] != ", reftags[key], " (", tags[key], ")")
			# Compare floats
			if not is_equal_approx(tags["@ombb00"], 0.52373960893601):
					printerr("tags[@ombb00] != 0.52373960893601")
