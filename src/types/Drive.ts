export type DriveType =
  | 'Unknown'
  | 'NoRootDir'
  | 'Removable'
  | 'Fixed'
  | 'Remote'
  | 'CDRom'
  | 'RamDisk';

export type Drive = {
  name: string;
  drive_type: DriveType;
  sectors_per_cluster: number;
  bytes_per_sector: number;
  number_of_free_clusters: number;
  number_of_clusters: number;
};
