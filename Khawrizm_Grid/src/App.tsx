import { useEffect, useRef } from 'react';
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

import '@univerjs/design/lib/index.css';
import '@univerjs/ui/lib/index.css';
import '@univerjs/sheets-ui/lib/index.css';

export default function App() {
  const containerRef = useRef(null);

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
      id: 'universal-sheet',
      name: 'Universal Grid',
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
        filters: [{ name: 'Universal Data', extensions: ['xcv', 'csv'] }],
      });
      if (filePath) {
        // سيتم ربط هذا لاحقاً لسحب البيانات من النواة
        await writeTextFile(filePath, "UNIVERSAL_SECURE_DATA_DUMP");
        alert('Saved Successfully (Air-Gapped)!');
      }
    } catch (err) {
      console.error(err);
    }
  };

  const handleOpenLocal = () => {
      alert('Local File Interface Ready...');
  };

  return (
    <div dir="ltr" style={{ display: 'flex', flexDirection: 'column', height: '100vh', background: '#f5f5f5' }}>
      <div style={{ padding: '8px 16px', backgroundColor: '#ffffff', borderBottom: '1px solid #e0e0e0', display: 'flex', gap: '12px', alignItems: 'center', fontFamily: 'system-ui, sans-serif' }}>
        <div style={{ fontWeight: 600, fontSize: '14px', marginRight: '16px', color: '#333' }}>
          Universal Grid
        </div>
        <button onClick={handleOpenLocal} style={{ padding: '6px 12px', cursor: 'pointer', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', fontSize: '13px' }}>
          Open File
        </button>
        <button onClick={handleSaveLocal} style={{ padding: '6px 12px', cursor: 'pointer', background: '#005fb8', color: '#fff', border: '1px solid #005fb8', borderRadius: '4px', fontSize: '13px' }}>
          Save Locally
        </button>
      </div>
      <div ref={containerRef} id="univer-container" style={{ flex: 1, width: '100%', overflow: 'hidden' }} />
    </div>
  );
}
