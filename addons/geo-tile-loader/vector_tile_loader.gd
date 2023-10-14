static func load_tile(fn) -> VectorTile:
	var bytes = FileAccess.get_file_as_bytes(fn)
	return VectorTile.read(bytes)

static func load_tile_gz(fn) -> VectorTile:
	var bytes_gz = FileAccess.get_file_as_bytes(fn)
	var bytes = bytes_gz.decompress_dynamic(-1, FileAccess.COMPRESSION_GZIP)
	return VectorTile.read(bytes)
