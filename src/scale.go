package main


import (
	"bytes"
	"fmt"

	"C"
	"github.com/centrifuge/go-substrate-rpc-client/scale"
)

//export TestFFI
func TestFFI() *C.char {
	return C.CString("World")
}

//export Multiplication
func Multiplication(a uint32, b uint32) uint32 {
	return a * b
}

//export EncodeString
func EncodeString(str *C.char) *C.char {
	var buf = bytes.Buffer{}

	err := scale.NewEncoder(&buf).Encode(C.GoString(str))
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String())
}

//export DecodeString
func DecodeString(data *C.char) *C.char {
	r := bytes.NewBufferString(C.GoString(data))
	var resp string
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.CString(resp)
}

//export EncodeI8
func EncodeI8(n C.schar) *C.char {
	var buf = bytes.Buffer{}
	err := scale.NewEncoder(&buf).Encode(n)
	if err != nil {
		panic(err)
	}
	return C.CString(buf.String())
}

//export DecodeI8
func DecodeI8(data *C.char) C.schar {
	r := bytes.NewBufferString(C.GoString(data))
	var resp int8
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.schar(resp)
}

//export EncodeU16
func EncodeU16(n C.ushort) *C.char {
	var buf = bytes.Buffer{}
	err := scale.NewEncoder(&buf).Encode(n)
	if err != nil {
		panic(err)
	}
	return C.CString(buf.String())
}

//export DecodeU16
func DecodeU16(data *C.char) C.ushort {
	r := bytes.NewBufferString(C.GoString(data))
	var resp uint16
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.ushort(resp)
}

//export EncodeU32
func EncodeU32(n C.uint) *C.char {
	var buf = bytes.Buffer{}
	err := scale.NewEncoder(&buf).Encode(n)
	if err != nil {
		panic(err)
	}

	return C.CString(buf.String())
}

//export DecodeU32
func DecodeU32(data *C.char) C.uint {
	r := bytes.NewBufferString(C.GoString(data))
	var resp uint32
	err := scale.NewDecoder(r).Decode(&resp)
	if err != nil {
		panic(err)
	}

	return C.uint(resp)
}

func main() {

	//Test encode decode int8
	char := EncodeU32(16777215)
	fmt.Println("EncodeU32",char)

	ch := DecodeU32(char)
	fmt.Println("DecodeU32",ch)

}
