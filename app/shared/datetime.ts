export function fmt_timestamp(ts: number): string {
    return new Date(ts * 1e3).toLocaleString('fa-ir')
}
