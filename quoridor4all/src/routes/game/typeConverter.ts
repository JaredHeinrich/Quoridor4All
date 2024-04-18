
export function wallToRust(wall: {
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}): {
  position: {
    x: number, 
    y: number
  },
  is_horizontal: boolean
}{
  return {
    position: wall.position,
    is_horizontal: wall.isHorizontal
  }.
}