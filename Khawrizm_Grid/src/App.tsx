import { useEffect, useRef, useState } from 'react';
import { Univer, UniverInstanceType, LocaleType } from '@univerjs/core';
import { UniverDocsPlugin } from '@univerjs/docs';
import { UniverDocsUIPlugin } from '@univerjs/docs-ui';
import { UniverUIPlugin } from '@univerjs/ui';
import { UniverSheetsPlugin } from '@univerjs/sheets';
import { UniverSheetsUIPlugin } from '@univerjs/sheets-ui';
import { UniverSheetsFormulaPlugin } from '@univerjs/sheets-formula';
import { UniverFormulaEnginePlugin } from '@univerjs/engine-formula';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

import '@univerjs/design/lib/index.css';
import '@univerjs/ui/lib/index.css';
import '@univerjs/sheets-ui/lib/index.css';

export default function App() {
  const containerRef = useRef(null);
  const [formula, setFormula] = useState('');

  useEffect(() => {
    if (!containerRef.current) return;

    const univer = new Univer({
      locale: LocaleType.EN_US,
    });

    univer.registerPlugin(UniverDocsPlugin, { hasScroll: false });
    univer.registerPlugin(UniverDocsUIPlugin);
    univer.registerPlugin(UniverUIPlugin, {
      container: containerRef.current,
      header: true,
      toolbar: true,
      footer: true,
    });
    univer.registerPlugin(UniverSheetsPlugin);
    univer.registerPlugin(UniverSheetsUIPlugin);
    univer.registerPlugin(UniverFormulaEnginePlugin);
    univer.registerPlugin(UniverSheetsFormulaPlugin);

    univer.createUnit(UniverInstanceType.UNIVER_SHEET, {
      id: 'sovereign-grid',
      name: 'Khawrizm Absolute Grid',
      sheetOrder: ['sheet1'],
      sheets: {
        'sheet1': {
          id: 'sheet1',
          name: 'Sheet 1',
          cellData: {},
        },
      },
    });

    return () => univer.dispose();
  }, []);

  const handleSaveLocal = async () => {
    try {
      const filePath = await save({
        filters: [{ name: 'Khawrizm Sovereign Data', extensions: ['xcv', 'csv'] }],
      });
      if (filePath) {
        await writeTextFile(filePath, '{"status": "SECURE_DATA_EXTRACTED_OFFLINE"}');
        alert('Sovereign Data Saved Locally (Air-Gapped)!');
      }
    } catch (err) {
      console.error(err);
    }
  };

  const executeFormula = async () => {
    if (!formula) return;
    try {
      // IPC Call bypassing standard JS evaluation -> sending directly to Rust
      const result = await invoke('evaluate_xcv', { formula });
      alert(result);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div dir="ltr" style={{ display: 'flex', flexDirection: 'column', height: '100vh', background: '#0a0a0a' }}>
      <div style={{ padding: '12px 20px', backgroundColor: '#0a0a0a', borderBottom: '1px solid #00ff88', display: 'flex', gap: '15px', alignItems: 'center', fontFamily: 'monospace' }}>
        <div style={{ fontWeight: 'bold', fontSize: '16px', color: '#00ff88', textShadow: '0 0 5px #00ff88', letterSpacing: '1px' }}>
          KHAWRIZM IPC
        </div>
        <input 
          type="text" 
          value={formula}
          onChange={(e) => setFormula(e.target.value)}
          placeholder="Ring-0 Equation (e.g., =SUM(A1:A5))" 
          style={{ flex: 1, padding: '8px 12px', background: '#111', color: '#00ff88', border: '1px solid #333', borderRadius: '2px', outline: 'none', fontFamily: 'monospace' }}
          onKeyDown={(e) => e.key === 'Enter' && executeFormula()}
        />
        <button onClick={executeFormula} style={{ padding: '8px 16px', cursor: 'pointer', background: '#00ff88', color: '#0a0a0a', border: 'none', fontWeight: 'bold', borderRadius: '2px', textTransform: 'uppercase' }}>
          Execute
        </button>
        <button onClick={handleSaveLocal} style={{ padding: '8px 16px', cursor: 'pointer', background: '#111', color: '#00ff88', border: '1px solid #00ff88', fontWeight: 'bold', borderRadius: '2px', textTransform: 'uppercase' }}>
          Save (Air-Gap)
        </button>
      </div>
      <div ref={containerRef} id="univer-container" style={{ flex: 1, width: '100%', overflow: 'hidden' }} />
    </div>
  );
}
