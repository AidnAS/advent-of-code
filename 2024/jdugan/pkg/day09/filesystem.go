package day09

import (
	"slices"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type FileSystem struct {
	files []File
}

// ========== RECEIVERS ===================================

func (fs *FileSystem) CompressBlocks() int {
	disk := fs.ToDisk()
	spaces := disk.SpaceIndices()
	values := disk.ValueIndices()
	slices.Reverse(values)

	curr_value, rem_values := pie.Shift(values)
	curr_space, rem_spaces := pie.Shift(spaces)
	for curr_value > curr_space {
		disk.bytes[curr_space] = disk.bytes[curr_value]
		disk.bytes[curr_value] = -1
		curr_value, rem_values = pie.Shift(rem_values)
		curr_space, rem_spaces = pie.Shift(rem_spaces)
	}

	return disk.CheckSum()
}

func (fs *FileSystem) CompressFiles() int {
	disk := fs.ToDisk()
	for fid := len(fs.files) - 1; fid >= 0; fid-- {
		vidxs := disk.ValueIndicesForId(fid)
		vlen := len(vidxs)
		sidxs := disk.SpaceIndicesByLength(vlen)
		if len(sidxs) > 0 && vidxs[0] > sidxs[0] {
			for i := 0; i < vlen; i++ {
				disk.bytes[sidxs[i]] = disk.bytes[vidxs[i]]
				disk.bytes[vidxs[i]] = -1
			}
		}
	}
	return disk.CheckSum()
}

// ---------- UTILITIES -----------------------------------

func (fs FileSystem) ToDisk() Disk {
	bytes := make([]int, 0)
	for _, file := range fs.files {
		for i := 0; i < file.block_size; i++ {
			bytes = append(bytes, file.id)
		}
		for i := 0; i < file.free_size; i++ {
			bytes = append(bytes, -1)
		}
	}
	return Disk{bytes: bytes}
}
