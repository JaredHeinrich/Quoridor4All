
export function wallToRust(wall: {
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}|null): {
  position: {
    x: number, 
    y: number
  },
  is_horizontal: boolean
}|null{
  if(wall)
  return {
    position: wall.position,
    is_horizontal: wall.isHorizontal
  };
  else
  return null
}