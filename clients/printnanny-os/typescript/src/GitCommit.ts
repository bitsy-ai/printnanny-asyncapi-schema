
interface GitCommit {
  oid: string;
  header: string;
  message: string;
  ts: number;
}
export { GitCommit };