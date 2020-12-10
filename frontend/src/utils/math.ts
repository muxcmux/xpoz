export type Bounds = {
  min: Point,
  max: Point,
}

export type Point = {
  x: number,
  y: number,
}

export function clamp(number: number, min: number, max: number): number {
  return Math.min(Math.max(number, min), max);
}

export function between(number: number, lower: number, upper: number): boolean {
  return number > lower && number < upper;
}

export function outOfBounds(point: Point, bounds: Bounds): Point | false {
  if (point.x < bounds.min.x
     || point.y < bounds.min.y
     || point.x > bounds.max.x
     || point.y > bounds.max.y) {
       let x = 0;
       let y = 0;
       if (bounds.max.x < point.x) {
         x = bounds.max.x - point.x;
       } else if (bounds.min.x > point.x) {
         x = bounds.min.x - point.x;
       }
       if (bounds.max.y < point.y) {
         y = bounds.max.y - point.y;
       } else if (bounds.min.y > point.y) {
         y = bounds.min.y - point.y;
       }
       return { x: Math.round(x), y: Math.round(y) };
     }
  return false;
}

export function roundPoint({x, y}: Point): Point {
  return { x: Math.round(x), y: Math.round(y) }
}

export function roundBounds({min, max}: Bounds): Bounds {
  return {
    min: { x: Math.round(min.x), y: Math.round(min.y) },
    max: { x: Math.round(max.x), y: Math.round(max.y) },
  }
}
