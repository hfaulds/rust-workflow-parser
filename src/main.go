package main

//#cgo LDFLAGS: -L${SRCDIR}/../target/debug -lworkflow_parser -ldl
//#include <workflow_parser.h>
import "C"
import (
	"fmt"
	"io/ioutil"
)

func main() {
	b, err := ioutil.ReadFile("example.yml")
	if err != nil {
		panic(err)
	}
	result := C.parse(C.CString(string(b)))
	if !is_result_ok(result) {
		panic(err_from_result(result))
	}
	fmt.Println(workflow_from_result(result))
}
