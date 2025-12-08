package main

import "testing"

func TestAdjustDial(t *testing.T) {
	tests := []struct {
		dial     int
		password int
		n        int
		wantDial int
		wantPass int
	}{
		{50, 0, 10, 60, 0}, // simple increment
		{95, 0, 10, 5, 1},  // wrap over 100
		{5, 0, -10, 95, 1}, // wrap below 0
		{0, 0, 0, 0, 0},    // dial at 0 triggers password increment
		{99, 0, 1, 0, 1},   // exactly to 100, then to 0
		{0, 2, 0, 0, 2},    // already at 0, n=0
		{50, 1, 150, 0, 3}, // 50+150=200->0, triggers password twice (100, then 0)
		{0, 1, -1, 99, 1},  // from 0 to -1 wraps to 99 and increments password
		{95, 0, 105, 0, 2}, // 95+105=200->0, triggers password twice (100, then 0)
		{5, 0, -105, 0, 2}, // 5-105=-100->0, triggers password twice (-100, then 0)
	}
	for _, tt := range tests {
		dial, pass := adjustDial(tt.dial, tt.password, tt.n)
		if dial != tt.wantDial || pass != tt.wantPass {
			t.Errorf("adjustDial(%d, %d, %d) = (%d, %d); want (%d, %d)", tt.dial, tt.password, tt.n, dial, pass, tt.wantDial, tt.wantPass)
		}
	}
}
