export class Smoother {
  timestampSeconds: number
  secondsToHalve: number
  value: number

  constructor(msToHalve: number, value: number = 0) {
    this.timestampSeconds = 0
    // If we start with a value of 1 and then only update with the value 0, how long
    // will it take to reach 0.5?
    this.secondsToHalve = msToHalve
    this.value = value
  }

  update(newTimestampSeconds: number, newValue: number) {
    if (!newTimestampSeconds) return this.value

    // In case we get non-monotonic timestamps, handle it gracefully
    const secondsElapsed = Math.max(newTimestampSeconds - this.timestampSeconds, 0)
    this.timestampSeconds = newTimestampSeconds

    const halfTimesElapsed = secondsElapsed / this.secondsToHalve
    const oldValueCoef = 0.5 ** halfTimesElapsed
    // console.log(oldValueCoef, this.value, newValue)

    this.value = oldValueCoef * this.value + (1 - oldValueCoef) * newValue
    return this.value
  }
}
