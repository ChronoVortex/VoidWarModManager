using UndertaleModLib;
using UndertaleModLib.Project;

namespace UndertaleProjectMan
{
    public class ProjectInstaller
    {
        private UndertaleData? Data { get; set; }

        private void SaveDataFile(string outputPath)
        {
            try
            {
                // Save data.win to temp file
                using (FileStream fs = new(outputPath + "temp", FileMode.Create, FileAccess.Write))
                {
                    UndertaleIO.Write(fs, Data);
                }

                // If we're executing this, the saving was successful. So we can replace the new temp file
                // with the older file, if it exists.
                File.Move(outputPath + "temp", outputPath, true);
            }
            catch (Exception e)
            {
                // Delete the temporary file in case we partially wrote it
                if (File.Exists(outputPath + "temp"))
                    File.Delete(outputPath + "temp");
                throw new IOException($"Could not save data file: {e.Message}");
            }
        }

        private void ReadDataFile(string inputPath)
        {
            try
            {
                using FileStream fs = new FileStream(inputPath, FileMode.Open, FileAccess.Read);
                Data = UndertaleIO.Read(fs,
                    (string warning, bool isImportant) => Console.WriteLine($"[WARNING]: {warning}"),
                    (string message) => Console.WriteLine($"[MESSAGE]: {message}"));
            }
            catch (FileNotFoundException e)
            {
                throw new FileNotFoundException($"Data file '{e.FileName}' does not exist");
            }
        }

        public void InstallProject(string dataPath, string projectPath)
        {
            ReadDataFile(dataPath);
            ProjectContext project = ProjectContext.CreateWithDataFilePaths(dataPath, dataPath, projectPath);
            project.Import(Data, new GameFileNoOpBackup());
            SaveDataFile(dataPath);
        }
    }
}
