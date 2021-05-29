package main

import "C"
import (
	"bytes"
	"unsafe"

	"github.com/centrifuge/go-substrate-rpc-client/scale"
	"github.com/centrifuge/go-substrate-rpc-client/types"
)

func assert(pred bool) {
	if !pred {
		panic("assertion failed")
	}
}

//export EncodeString
func EncodeString(str *C.char) (*C.char, C.int) {
	var buf = bytes.Buffer{}
	err := scale.NewEncoder(&buf).Encode(C.GoString(str))
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeString
func DecodeString(data *C.char, length C.int) (*C.char, C.int) {
	goSLice := C.GoBytes(unsafe.Pointer(data), length)
	r := bytes.NewReader(goSLice)
	var resp string
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.CString(resp), C.int(len(resp))
}

//export EncodeI8
func EncodeI8(n C.schar) (*C.char, C.int) {
	var buf = bytes.Buffer{}
	err := scale.NewEncoder(&buf).Encode(n)
	if err != nil {
		panic(err)
	}
	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeI8
func DecodeI8(data *C.char, length C.int) C.schar {
	goSLice := C.GoBytes(unsafe.Pointer(data), length)
	r := bytes.NewReader(goSLice)
	var resp int8
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.schar(resp)
}

//export EncodeU16
func EncodeU16(n C.ushort) (*C.char, C.int) {
	var buf = &bytes.Buffer{}
	err := scale.NewEncoder(buf).Encode(n)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeU16
func DecodeU16(data *C.char, len C.int) C.uint {
	goSLice := C.GoBytes(unsafe.Pointer(data), len)
	r := bytes.NewReader(goSLice)
	var resp uint16
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.uint(resp)
}

//export EncodeU32
func EncodeU32(n C.uint) (*C.char, C.int) {
	var buf = &bytes.Buffer{}
	err := scale.NewEncoder(buf).Encode(n)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeU32
func DecodeU32(data *C.char, len C.int) C.uint {
	goSLice := C.GoBytes(unsafe.Pointer(data), len)
	r := bytes.NewReader(goSLice)
	var resp uint32
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.uint(resp)
}

//export EncodeVecU8
func EncodeVecU8(arr []uint8) (*C.char, C.int) {
	var buf = &bytes.Buffer{}
	err := scale.NewEncoder(buf).Encode(arr)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeVecU8
func DecodeVecU8(data *C.char, length C.int) (unsafe.Pointer, C.int) {
	goSLice := C.GoBytes(unsafe.Pointer(data), length)
	r := bytes.NewReader(goSLice)
	var arr []uint8
	err := scale.NewDecoder(r).Decode(&arr)
	if err != nil {
		panic(err)
	}

	return C.CBytes(arr), C.int(len(arr))
}

//export EncodeOptionBool
func EncodeOptionBool(hasValue C.uchar, value C.uchar) (*C.char, C.int) {
	assert(hasValue <= 1)

	var optBool types.OptionBool
	if hasValue == 0 {
		optBool = types.NewOptionBoolEmpty()
	} else {
		switch value {
		case 1:
			optBool = types.NewOptionBool(true)
		case 2:
			optBool = types.NewOptionBool(false)
		}
	}

	var buf = &bytes.Buffer{}
	encoder := scale.NewEncoder(buf)
	err := optBool.Encode(*encoder)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeOptionBool
func DecodeOptionBool(data *C.char, len C.int) (C.uchar, C.uchar) {
	goSLice := C.GoBytes(unsafe.Pointer(data), len)
	r := bytes.NewReader(goSLice)
	decoder := scale.NewDecoder(r)

	var optBool types.OptionBool
	err := optBool.Decode(*decoder)
	if err != nil {
		panic(err)
	}

	has, value := optBool.Unwrap()
	if !has {
		assert(value == false)
		return C.uchar(0), C.uchar(0)
	}

	if value {
		return C.uchar(1), C.uchar(1)
	}

	return C.uchar(1), C.uchar(2)
}

//export EncodeOptional
func EncodeOptional(hasValue C.uchar, value C.uchar) (*C.char, C.int) {
	assert(hasValue <= 1)

	var optBytes types.OptionU8
	if hasValue == 0 {
		optBytes = types.NewOptionU8Empty()
	} else {
		optBytes = types.NewOptionU8(types.NewU8(uint8(value)))
	}

	var buf = &bytes.Buffer{}
	encoder := scale.NewEncoder(buf)
	err := optBytes.Encode(*encoder)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String()), C.int(buf.Len())
}

//export DecodeOptional
func DecodeOptional(data *C.char, len C.int) (C.uchar, C.uchar) {
	goSLice := C.GoBytes(unsafe.Pointer(data), len)
	r := bytes.NewReader(goSLice)
	decoder := scale.NewDecoder(r)

	var optByte types.OptionU8
	err := optByte.Decode(*decoder)
	if err != nil {
		panic(err)
	}

	has, value := optByte.Unwrap()
	if !has {
		assert(value == 0)
		return C.uchar(0), C.uchar(0)
	}

	return C.uchar(1), C.uchar(value)
}

func main() {}
