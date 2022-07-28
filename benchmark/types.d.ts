declare module "tarts" {
  export default function Tarts(
    files: { name: string; content: Uint8Array }[]
  ): Uint8Array;
}
