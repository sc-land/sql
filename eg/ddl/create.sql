CREATE TABLE Enzyme (
                        registry_number INT PRIMARY KEY,
                        substrate TEXT,
                        optimal_ph INT,
                        temperature_sensitive BOOLEAN,
                        active_site_count INT
);
CREATE TABLE Genome (
                        accession_code INT PRIMARY KEY,
                        chromosome_count INT,
                        gc_content INT,
                        is_circular BOOLEAN
);
CREATE TABLE Bug (
                        specimen_tag INT PRIMARY KEY,
                        wing_span INT,
                        colony_role TEXT,
                        venomous BOOLEAN,
                        antenna_length INT,
                        metamorphosis_complete BOOLEAN
);
