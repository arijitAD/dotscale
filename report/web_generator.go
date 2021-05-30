package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
)

const (
	head = `
		<!DOCTYPE html>
		<html lang="en">
		<head>
		  <title>SCALE Codec Compatibility Report
		</title>
		  <meta charset="utf-8">
		  <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.4.1/css/bootstrap.min.css">
		  <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.5.1/jquery.min.js"></script>
		  <script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.4.1/js/bootstrap.min.js"></script>
		</head>
	`
)

type Result struct {
	Type        string  `json:"type,omitempty"`
	Name        string  `json:"name,omitempty"`
	Event       string  `json:"event,omitempty"`
	Passed      int     `json:"passed,omitempty"`
	Failed      int     `json:"failed,omitempty"`
	AllowedFail int     `json:"allowed_fail,omitempty"`
	Ignored     int     `json:"ignored,omitempty"`
	Measured    int     `json:"measured,omitempty"`
	FilteredOut int     `json:"filtered_out,omitempty"`
	ExecTime    float64 `json:"exec_time,omitempty"`
	TestCount   int     `json:"test_count,omitempty"`
}

func lineReader(path string) ([]*Result, error) {
	curr, err := os.Getwd()
	if err != nil {
		return nil, err
	}

	filePath := filepath.Join(curr, path)

	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}

	r := bufio.NewReader(file)

	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	var result []*Result
	for scanner.Scan() {
		var res Result
		err = json.Unmarshal([]byte(scanner.Text()), &res)
		if err != nil {
			return nil, err
		}
		result = append(result, &res)
	}

	return result, nil
}

func generateHtml(inFile, outFile string) {
	resultStr, err := lineReader(inFile)
	if err != nil {
		log.Fatal(err)
	}

	lastRes := resultStr[len(resultStr)-1]

	detailsTable := fmt.Sprintf(`
	  <div class="col-sm-4 col-md-4 col-lg-4">
	  <table class="table table-striped table-responsive-md">
		<thead>
		  <tr>
			<th>Library Details</th>
			<th></th>
			<th></th>
		  </tr>
		</thead>
		<tbody>
		  <tr>
			<td>Scale Implementation</td>
			<td colspan="2"><a href="https://github.com/centrifuge/go-substrate-rpc-client/blob/master/scale/codec.go">go-substrate-rpc-client</a></td>
		  </tr>
		  <tr>
			<td>Total test count:</td>
			<td colspan="2">%d</td>
		  </tr>
		  <tr>
			<td>Passed</td>
			<td colspan="2">%d</td>
		  </tr>
		  <tr>
			<td>Failed</td>
			<td colspan="2">%d</td>
		  </tr>
		  <tr>
			<td>Execution Time (in secs)</td>
			<td colspan="2">%.10f</td>
		  </tr>
		</tbody>
	  </table>
	  </div>`,
		resultStr[0].TestCount,
		lastRes.Passed,
		lastRes.Failed,
		lastRes.ExecTime,
	)

	testTable := `
	  <div class="col-sm-8 col-md-8 col-lg-8">
	  <table class="table table-responsive-md">
		<thead>
		  <tr>
			<th scope="col">#</th>
			<th>Test Name</th>
			<th>Status</th>
			<th>Execution Time (in secs)</th>
		  </tr>
		</thead>
		<tbody>`

	var count int64
	for _, v := range resultStr {
		if v.Event == "started" || v.Type == "suite" {
			continue
		}
		count++

		if v.Event == "ok" {
			testTable = testTable + fmt.Sprintf(`
				<tr class="success">
			   	<th scope="row">%d</th>
			   	<td>%s</td>
				<td><span class="badge badge-pill badge-success">Success</span></td>
			   	<td>%.10f</td>
				</tr>
				`, count, v.Name, v.ExecTime)
			continue
		} else {
			testTable = testTable + fmt.Sprintf(`
				<tr class="danger">
			   	<th scope="row">%d</th>
			   	<td>%s</td>
				<td><span class="badge badge-pill badge-danger">Failed</span></td>
			   	<td>%.10f</td>
				</tr>
				`, count, v.Name, v.ExecTime)
		}
	}

	testTable = testTable + `</tbody></table></div>`

	bodyHtml := fmt.Sprintf(`
		<body>
		<div class="container">
		  <h2 class="text-center">SCALE Codec Compatibility Report</h2><br>
		%s <! -- Details Table -->
		%s <! -- Test Table -->
		</div>
		</body>
		</html>
	`, detailsTable, testTable)

	writeToFile(outFile, fmt.Sprintf(`%s %s`, head, bodyHtml))
}

func writeToFile(outFile, data string) {
	f, err := os.Create(outFile)
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	_, err = f.WriteString(data)
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	if len(os.Args) != 3 {
		log.Fatal("Invalid command: go run ./report/web_generator.go <input json> <output html>")
	}
	inputFile, outputFile := os.Args[1], os.Args[2]
	generateHtml(inputFile, outputFile)
}
