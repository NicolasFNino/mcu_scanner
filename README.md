# mcu_scanner  
## Project overview:  

The main goal of the project is to create a tool that analyzes Over-the-Air (OTA) firmware update images and identifying details about the vendor, chip, family, and architecture of the image. This tool would do this by reading metadata (image header) to identify the firmware, extract the metadata, and verify that the input binary file is a valid firmware image.  The two main functions this tool would allow for are to:  
1. Verify whether the binary file is a valid firmware image that can be flashed to a certain chip using vendor-specific signatures. If these signatures are identified, then it is assumed that the image is a valid firmware image.  
2. Show the user details that can be used for security analysis, such as program entry address.  

A comprehensive list of features that will be included in this tool are as follows:  
(1) Firmware Identification  
	a. Analyse the firmware image to detect the image header.  
	b. Identify the relevant vendor information.  
	c. Identify the chip family and architecture of the firmware.  
	d. The tool will attempt to validate whether vendor-related data, such as firmware signatures, have been altered during 				transmission. In some cases, this is possible because certain vendors include a CRC field in their metadata. We can use this 	value to verify the integrity of the image by emulating the vendor's process.  
(2) Metadata Extraction:  
	a. Parse and extract metadata related to SDK versions, platform specifics, and image validation.
	b. Display structured information to the user, making it easier to verify the firmware origin and contents.  
(3) Binary Validation:  
	a. Verify whether the input binary file is a valid firmware image.  
		i. The tool will verify that the firmware image follows the expected format using checksums embedded in the header and checking the format integrity/vendor-specific markers.  
		ii. Make sure image size, layout, and embedded security features are correct and follow the firmware specifications.  


## Architecture:  

### Firmware Extraction Module:  
Sometimes OTA firmware update images are transported in an encoded format such as Intel Hex or Motorola S-Record, to save space during transfer. This module will take care of:  
1. Identifying if the image is encoded as one of the known formats  
2. Decoding the file to turn it into a raw binary blob  
3. Report the first section of the file to the next mosule for analysis  

### Firmware Identification and Validation Module:  
Part 1:  
This part of our tool will be able to identify any key details about the firmware image by looking for the image header, which contains all the important information. This information can be used for identification. Because this part of the tool will be looking at the header, which contains all this information, it will be able to tell what vendor made it and what hardware it was made for. This will help us make sure that the firmware is compatible and allow us to move forward to the next stage.  
Part 2:  
In the next stage, the tool will check to see if the image is valid and make sure that it has not been modified by doing checks on the integrity of the image using checksum, header validation, etc. It will also check for signatures to prove authenticity and ensure that it is the correct size/layout and has the appropriate security. In summation, it will prove that the firmware is not corrupt or was not modified.  
Part 3:  
The Last stage is the metadata extraction part, which is where we will be extracting all the useful information about the firmware, including the SDK version and specific details about the platform/features/requirements about the hardware. It will do this by taking all the data and showing it in a clean and structured format that will allow the user to see all the important information.  


## Dependencies:  