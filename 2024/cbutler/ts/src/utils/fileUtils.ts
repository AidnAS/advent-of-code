import * as fs from 'fs';
import * as path from 'path';


interface IFileReader {
	filename?: string;
	date?: string;
	useTestData?: boolean;
	part?: number;
	verbose?: boolean;
}

export const dataFileReader = ({ filename = '', date = '', useTestData = false, part = 1, verbose = false}: IFileReader): string[] => {

	let filepath = useTestData ? 
		`../common/data/${date}-part${part}-test-data.txt` : 
		`../common/data/${date}-data.txt`;
	
	if (filename !== '') filepath = filename;

	const absolutePath = path.resolve(filepath);
	const fileContents = fs.readFileSync(absolutePath, 'utf-8');
	const lines = fileContents.split(/\r?\n/);
	return lines;
}