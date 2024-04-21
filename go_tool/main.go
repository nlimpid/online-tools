//go:build js && wasm

package main

import (
	"fmt"
	"syscall/js"
	"time"

	"log/slog"

	"github.com/robfig/cron/v3"
)

func main() {
	done := make(chan struct{}, 0)
	global := js.Global()

	global.Set("wasmGenerateCron", js.FuncOf(generateCron))
	<-done
}

var (
	defaultCronParse = cron.NewParser(
		cron.SecondOptional | cron.Minute | cron.Hour | cron.Dom | cron.Month | cron.Dow | cron.Descriptor,
	)
)

func generateCron(this js.Value, args []js.Value) any {
	slog.Info("this is %v, args is %v", this.Type().String(), args[0])
	inputTime := args[0].String()

	s, err := defaultCronParse.Parse(inputTime)
	if err != nil {
		return fmt.Errorf("parse err %v", err).Error()
	}
	return s.Next(time.Now()).Format(time.DateTime)
}
