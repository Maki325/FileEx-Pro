import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import byteSize from 'byte-size';
import { Drive } from './types';

function App() {
  const [drives, setDrives] = useState<Drive[]>([]);
  const [selectedDrive, setSelectedDrive] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      setDrives(await invoke('list_drives'));
    })();
  }, []);

  function getDriveUsage(drive: Drive) {
    const bytes_per_cluster =
      drive.sectors_per_cluster * drive.bytes_per_sector;
    return `${byteSize(drive.number_of_free_clusters * bytes_per_cluster, {
      units: 'iec',
    })} free of ${byteSize(drive.number_of_clusters * bytes_per_cluster, {
      units: 'iec',
    })}`;
  }

  return (
    <div className="container">
      <h1>FileEx Pro</h1>

      <div className="drives noselect">
        {drives.map((drive) => (
          <div
            data-selected={selectedDrive === drive.name}
            key={drive.name}
            className="drive"
            onClick={() => setSelectedDrive(drive.name)}>
            <img src="/drive.ico" alt="Drive" />
            <div>
              <h1 key={drive.name}>{drive.name}</h1>
              <h4>{getDriveUsage(drive)}</h4>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
